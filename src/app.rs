use dioxus::prelude::*;
use rfd::FileDialog;

pub fn App(cx: Scope) -> Element {
    let home_dir = dirs::home_dir().unwrap();
    let mut picked_pathbuf = std::path::PathBuf::new();
    let picked_path = use_state(&cx, || "".to_string());

    cx.render(rsx!(
        style { [include_str!("css/global.css")] }
        h1 {
            style: "margin-top: 5vh;",
            "Convi"
        }
        h5 {
            style: "margin-top: 15vh;margin-bottom:1vh;",
            "Input file:"
        }
        p {
            // style: "",
            [
                if picked_path.get() != "" {
                    format!("{}", picked_path.get())
                } else {
                    "None".to_string()
                }
            ]
        }
        button {
            onclick: move |_| {
                let picked_pb = &mut picked_pathbuf;
                let opt = FileDialog::new()
                    .set_directory(&home_dir)
                    .pick_file();
                if let Some(p) = opt {
                    *picked_pb = p;
                    picked_path.set(picked_pb.to_str().unwrap().to_string());
                }
            },
            "Set input file"
        }
    ))
}
