use core::time;
use std::{error::Error, fs, time::Instant};
use select::{
    document::Document,
    predicate::{Attr, Name}
};

use scraper::{Html, Selector};

use tokio::main;

#[tokio::main]
async fn main() {
    bestiario().await
}

async fn bestiario() {
    const base_url: &str = "https://www.eliteguias.com/guias/d/dq11/dragon-quest-xi-ecos-de-un-pasado-perdido_monstruos-derrotados";
    let now = Instant::now();
    let mut i = 1;
    loop {
        let text: String = match reqwest::get(format!("{}-p{}.php", base_url, i)).await {
            Err(e) => {
                println!("{}", e);
                return;
            },
            Ok(r) => r.text().await.unwrap()
        };
    
        let d = Document::from(&text[..]);
        drop(text);
        
        let v = d.find(Name("div")).filter(|n| n.attr("id") == Some("guias")).collect::<Vec<_>>();
        let guia = match v.first() {
            None => {
                println!("[ERROR] Guía no encontrada.");
                return;
            }
            Some(g) => g
        };
    
        let mut content = String::new();
        let titulo = match guia.children().skip_while(|n| !n.text().contains("Monstruos derrotados")).skip(1).next() {
            None => {
                println!("[ERROR] Título no encontrado.");
                return;
            }
            Some(t) => t.text()
        };

        for c in guia.children() {
            if c.inner_html().contains("img") { continue; }
    
            content.push_str(&c.text());
            content.push('\n');
        }

        let out = format!("out/p{:0>3}-{}.txt", i, titulo.replace(" ", "_"));

        // Save file.
        match fs::write(
            std::path::Path::new(&out), 
            content
        ) {
            Ok(_) => eprintln!("[SUCCESS] Elapsed: {} - Saved: {}", now.elapsed().as_secs_f64(), out),
            Err(e) => eprintln!("[ERROR] Failed to save {}: {}", out, e)
        }

        i += 1;
        std::thread::sleep(time::Duration::from_secs_f64(1.5));
    }
}