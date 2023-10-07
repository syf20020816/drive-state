//! # error for state
//! ```txt
//! @author:syf20020816@Outlook.com
//! @date:2023/9/29
//! @version:0.0.1
//! @description:
//! code :
//! - 1001 : TranslationError 表示当前无法将一个状态转移到另一个状态，这是一种无法支持操作的错误类型
//! ```

use std::path::PathBuf;
use except_plugin::{UnSupportedOpException, ExceptionFactory, UnSupportedOpExceptionBuilder, SuperBuilderImpl, ExceptionLevel, CommonParamImpl, ReasonParamImpl, Reasons};

/// # TranslationError
pub struct TranslationError(UnSupportedOpException);

impl TranslationError {
    pub fn new(line: u32, file: &str) -> Self {
        let e = ExceptionFactory::new::<UnSupportedOpException, UnSupportedOpExceptionBuilder>()
            .set_code(1001)
            .set_level(ExceptionLevel::Fatal)
            .set_line(line)
            .set_path(PathBuf::from(file))
            .set_msg("translation error current state can not be translated")
            .set_reason(Reasons::Other(String::from("Unsupported")))
            .build();

        TranslationError(e)
    }

}