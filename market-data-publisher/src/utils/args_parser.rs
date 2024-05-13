use shared::serializer::SerializationType;

pub struct Args {
    pub serializer_type: SerializationType,
}

pub(crate) fn parse(args: Vec<String>) -> Args {
    let serializer_type = match args.len() {
        2 => {
            let cmd = &args[1];
            // parse the command
            match &cmd[..] {
                "json" => SerializationType::Json,
                "msgpk" => SerializationType::Messagepack,
                _ => SerializationType::Json,
            }
        }
        _ => SerializationType::Json,
    };
    Args { serializer_type }
}
