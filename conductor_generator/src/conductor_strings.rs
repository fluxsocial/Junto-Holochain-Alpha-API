pub static GENERAL_CONDUCTOR_DATA: &str = "
persistence_dir = \"/holochain/persistence\"
#signing_service_uri = \"http://localhost:8888\"

[[dnas]]
file = \"/holochain/dnas/Junto/junto-rust/dist/junto-rust.dna.json\"
id = \"junto-app\"
hash = \"Qmdu6RjFXeYfAL8MzpGG9RGUPReo36FLeNW4bZFpVkcY2N\"

#[[dnas]]
#file = \"/holochain/dnas/DeepKey/dist/DeepKey.dna.json\"
#id = \"deepkey\"
#hash = \"QmdEqRWmJ7MGfxQVKJcqdzghQ19ynK7CanUeTQFMoFeiPo\"

[network]
type=\"n3h\"
n3h_persistence_path = \"/holochain/n3h\"
n3h_log_level = 't'
#bootstrap_nodes = []
#n3h_mode = \"REAL\"
#Agent for hosting applications";

pub static INTERFACE_GENERAL: &str = "
[[interfaces]]
id = \"http interface\"
admin = true";

pub static INTERFACE_FINAL: &str = "
\t[interfaces.driver]
\ttype = \"http\"
\tport = 4000";
