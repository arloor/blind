use clap::{Args, Parser, Subcommand};

/// A tool that utilize git update-index --assume-unchanged to ignore files.
#[derive(Debug, Parser)]
#[clap(name = "blind", version)]
pub struct App {
    #[clap(flatten)]
    pub(crate) global_opts: GlobalOpts,

    #[clap(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// 忽略文件
    Ignore {
        /// 不展示当前被忽略的文件
        #[clap(long = "no-show")]
        no_show_ignored: bool,
        /// 需要被忽略的文件
        path: String,
    },
    /// 跟踪文件
    Watch(WatchArgs),
    /// 展示当前被忽略的文件
    List,
}

#[derive(Debug, Args)]
pub(crate) struct WatchArgs {
    /// 需要被跟踪的文件
    pub(crate) path: String,
    /// 不展示当前被忽略的文件
    #[clap(long = "no-show")]
    pub(crate) no_show_ignored: bool,
}

#[derive(Debug, Args)]
pub(crate) struct GlobalOpts {
    /// 是否打印调试信息
    #[clap(short, global = true, default_value = "false")]
    pub(crate) verbose: bool,
}
