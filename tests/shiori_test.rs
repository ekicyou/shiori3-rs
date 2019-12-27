#[test]
fn shiori_test1() {
    use futures::executor::ThreadPool as Exec;
    use futures::prelude::*;
    use futures::task::SpawnExt;
    use shiori3::*;
    use std::path::Path;

    // 実行エンジン
    let exec = Exec::new().unwrap();

    // イベントレシーバの取得(load)
    let mut event_receiver = {
        entry::set_hinst(1234);
        let g_load_dir = gstr::clone_from_path_nofree(Path::new("test"));
        let (hdir, len) = g_load_dir.value();
        entry::load(hdir, len).expect("entry::load")
    };

    // イベントループの実行
    let _ = exec
        .spawn(async move {
            loop {
                match event_receiver.next().await {
                    None => {
                        break;
                    }
                    Some(Event::Load(args)) => {
                        let (hinst, load_dir) = args.value();
                        println!("hinst={}, load_dir={}", hinst, load_dir);
                    }
                    Some(Event::Unload(args)) => {
                        let res = args.value();
                        res.done(Ok(())).expect("done");
                    }
                    Some(Event::Request(args)) => {
                        let (req, res) = args.value();
                        assert_eq!(req.as_ref(), "REQ1");
                        let res_text = format!("RES:{}", req.as_ref());
                        res.done(Ok(res_text)).expect("done");
                    }
                }
            }
        })
        .unwrap();

    // REQUEST
    {
        let g_req = gstr::clone_from_str_nofree("REQ1");
        let (h_req, mut len) = g_req.value();
        let h_res = entry::request(h_req, &mut len);
        let g_res = gstr::capture_str(h_res, len);
        assert_eq!(g_res.as_ref(), "RES:REQ1");
    }
}
