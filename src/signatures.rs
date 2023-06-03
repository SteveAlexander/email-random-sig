use osascript::JavaScript;
use serde_derive::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Debug)]
pub struct Signature {
    name: String,
    content: String,
}

impl Signature {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
        }
    }

    fn with_prefix(self, prefix: &str) -> Self {
        Self {
            name: prefix.to_owned() + &self.name,
            ..self
        }
    }
}

#[derive(Serialize, Debug)]
struct Signatures {
    signatures: Vec<Signature>,
}

#[derive(Deserialize, Debug)]
struct U32Result {
    #[serde(rename = "value")]
    value: u32,
}

// #[derive(Deserialize, Debug)]
// struct DebugResult {
//     #[serde(rename = "data")]
//     data: String
// }

#[derive(Serialize, Debug)]
struct SignatureNamePrefix {
    prefix: String,
}

pub fn add_signatures(prefix: &str, signatures: Vec<Signature>) -> Result<(), Box<dyn Error>> {
    let script = JavaScript::new(
        r###"
            var app = Application('Mail');
            $params.signatures.forEach(
                sig => app.signatures.push(app.Signature(sig))
            )
    "###,
    );
    let params = Signatures {
        signatures: signatures
            .into_iter()
            .map(|sig| sig.with_prefix(prefix))
            .collect(),
    };
    // println!("{params:?}");
    script.execute_with_params(params)?;
    Ok(())
}

pub fn add_signature(name: &str, content: &str) -> Result<u32, Box<dyn Error>> {
    let script = JavaScript::new(
        r###"
            var app = Application('Mail');
            var sig = app.Signature({
                name: $params.name,
                content: $params.content
            });
            var retval = app.signatures.push(sig);
            return { value: retval }; // returns index of added sig
    "###,
    );
    let params = Signature {
        name: name.into(),
        content: content.into(),
    };
    let result: U32Result = script.execute_with_params(params)?;
    Ok(result.value)
}

pub fn erase_all_signatures(prefix: &str) -> Result<u32, Box<dyn Error>> {
    let script = JavaScript::new(
        r###"
            var app = Application('Mail');
            var signatures = app.signatures();
            if (signatures === null) {
                signatures = [];
            }
            signaturesWithPrefix = signatures
                    .filter(sig => sig.name().startsWith($params.prefix));
            signaturesWithPrefix.forEach(sig => sig.delete());
            return { value: signaturesWithPrefix.length }; // returns number erased
    "###,
    );
    let params = SignatureNamePrefix {
        prefix: prefix.into(),
    };
    let result: U32Result = script.execute_with_params(params)?;
    Ok(result.value)
}

pub fn add_fixed_signature() -> Result<(), Box<dyn Error>> {
    let script = JavaScript::new(
        r###"
            var app = Application('Mail');
            var sig = app.Signature({
                name: 'Example signature',
                content: 'Example signature content'
            });
            app.signatures.push(sig);
        "###,
    );
    script.execute()?;
    Ok(())
}
