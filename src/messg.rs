#[derive(Clone,Copy,Debug)]
pub enum Message{
    New,
    OpenTerm,
    OpenFolder,
    Open,
    Close,
    Closed(i32),
    Save,
    SaveAs,
    SideBar(i32),
    Quit,
    None
}
