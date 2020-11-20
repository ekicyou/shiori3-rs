use crate::ApiResult;
use crate::GCowStr;
use crate::ShioriRequest;

pub struct ShioriRequestArgs {
    #[allow(dead_code)]
    src: GCowStr,
    req: ShioriRequest<'static>,
}

impl ShioriRequestArgs {
    pub fn new(src: GCowStr) -> ApiResult<ShioriRequestArgs> {
        let req = ShioriRequest::parse(&src)?;
        let req = unsafe { std::mem::transmute(req) };
        Ok(Self { src: src, req: req })
    }

    pub fn req(&self) -> &ShioriRequest<'_> {
        &self.req
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gstr;
    use crate::ShioriRequestRule;
    use log::*;

    #[test]
    fn req_args_1() {
        std::env::set_var("RUST_LOG", "trace");
        env_logger::init();
        trace!("test start");
        let src = include_str!("test_data/shiori3-1.txt")
            .replace("\r\n", "\n")
            .replace("\r", "\n")
            .replace("\n", "\r\n");
        let src = gstr::clone_from_str(&*src);

        let args = ShioriRequestArgs::new(src).unwrap_or_else(|e| panic!("{:?}", e));
        let req = args.req();
        assert_eq!(req.header.version, 30);
        assert_eq!(req.charset.unwrap(), "UTF-8");
        assert_eq!(req.sender.unwrap(), "SSP");
        assert_eq!(req.security_level.unwrap(), "local");
        assert_eq!(req.header.method, ShioriRequestRule::get);
        assert_eq!(req.id.unwrap(), "version");
        assert_eq!(req.status, None);
        assert_eq!(req.base_id, None);

        assert_eq!(req.dic.len(), 4);
        assert_eq!(req.dic["Charset"], "UTF-8");
        assert_eq!(req.dic["ID"], "version");
        assert_eq!(req.dic["SecurityLevel"], "local");
        assert_eq!(req.dic["Sender"], "SSP");

        assert_eq!(req.key_values.len(), 0);

        assert_eq!(req.reference.len(), 0);
    }
}
