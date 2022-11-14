use dioxus::prelude::*;
use rfd::FileDialog;

pub fn App(cx: Scope) -> Element {
    let home_dir = dirs::home_dir().unwrap();
    let mut picked_pathbuf = std::path::PathBuf::new();
    let picked_path = use_state(&cx, || "".to_string());
    let mut output_pathbuf = std::path::PathBuf::new();
    let output_path = use_state(&cx, || "".to_string());

    cx.render(rsx!(
        style { [include_str!("css/global.css")] }
        h1 {
            style: "margin-top: 4vh;",
            "Convi"
        }

        // Input
        h5 {
            style: "margin-top: 10vh;margin-bottom:1vh;",
            "Input file:"
        }
        p {
            // style: "margin-bottom: 1vh;",
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

        // Output
        h5 {
            style: "margin-top: 10vh;margin-bottom:1vh;",
            "Output file:"
        }
        p {
            // style: "",
            [
                if output_path.get() != "" {
                    format!("{}", output_path.get())
                } else {
                    "None".to_string()
                }
            ]
        }
        button {
            onclick: move |_| {
                let output_pb = &mut output_pathbuf;
                let opt = FileDialog::new()
                    .set_directory(&dirs::home_dir().unwrap())
                    .save_file();
                if let Some(p) = opt {
                    *output_pb = p;
                    output_path.set(output_pb.to_str().unwrap().to_string());
                }
            },
            "Set output file"
        }
    ))
}
