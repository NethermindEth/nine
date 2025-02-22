use anyhow::{anyhow, Result};
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next};
use crb::core::Slot;
use n9_core::{ChatRequest, Message, Particle, Prompt, Role, SubstanceBond, SubstanceLinks, Tool};
use schemars::{schema_for, JsonSchema};
use serde_json::Value;
use std::marker::PhantomData;

pub trait Toolkit: Default + Send + 'static {
    fn add_tools(
        &mut self,
        particle: &mut LiquidParticle<Self>,
        bond: &mut SubstanceBond<LiquidParticle<Self>>,
    );
}

pub struct LiquidParticle<K: Toolkit> {
    substance: SubstanceLinks,
    toolkit: PhantomData<K>,
    bond: Slot<SubstanceBond<Self>>,
}

impl<K> Particle for LiquidParticle<K>
where
    K: Toolkit,
{
    fn construct(substance: SubstanceLinks) -> Self {
        Self {
            substance,
            toolkit: PhantomData,
            bond: Slot::empty(),
        }
    }
}

impl<K> Agent for LiquidParticle<K>
where
    K: Toolkit,
{
    type Context = AgentSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl<K> DoAsync<Initialize> for LiquidParticle<K>
where
    K: Toolkit,
{
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let mut bond = self.substance.bond(&ctx);
        let mut toolkit = K::default();
        toolkit.add_tools(self, &mut bond);
        self.bond.fill(bond)?;
        Ok(Next::events())
    }
}

const TEMPLATE: &str = r#"
**Universal JSON Schema Tool**

- **Query Schema Placeholder:**
  Accepts a JSON schema for a query defined below.

- **Response Schema Placeholder:**
  Accepts a JSON schema for a response defined below.

- **Query Simulation:**
  Produces a valid JSON value that conforms to the defined query schema.

- **Random Response Generation:**
  Generates a random answer that adheres to the provided JSON schema for the response.

- **Schema Consistency:**
  Ensures that both input (query) and output (response) strictly follow their defined JSON schemas, promoting robust and predictable data exchange.

- **Tool Versatility:**
  Useful for testing, prototyping, and validating API endpoints and other systems that communicate using JSON.

## Tool description (behaviour to simulate as a tool)

{description}

## Query (prompt, input) schema

```json
{input}
```

## Response (reply, output) schema

```json
{output}
```

Respond only with a JSON value that conforms to the response schema.
"#;

#[async_trait]
impl<K, P> Tool<P> for LiquidParticle<K>
where
    K: Toolkit,
    P: Prompt,
{
    async fn call_tool(&mut self, input: P, _ctx: &mut Context<Self>) -> Result<P::Output> {
        let model = self.substance.router.get_model().await?;
        let input = schema::<P>()?;
        let output = schema::<P::Output>()?;
        let req = TEMPLATE
            .replace("{description}", P::description())
            .replace("{input}", &input)
            .replace("{output}", &output)
            .to_string();
        let message = Message::content(Role::Developer, req);
        let request = ChatRequest::from(message);
        let response = model.chat(request.into()).await?;
        let value = serde_json::from_str(&response.squash())?;
        Ok(value)
    }
}

fn schema<T>() -> Result<String>
where
    T: JsonSchema,
{
    let root_schema = schema_for!(T);
    let schema = serde_json::to_string(&root_schema.schema)?;
    Ok(schema)
}
