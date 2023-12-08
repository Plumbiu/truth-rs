use truth_rs_type::{AHashMap, RelationsMap};

use crate::util::merge_map;

enum Symbol {
    Tab,
    Vertical,
    Joiner,
    Line,
}

impl Symbol {
    fn as_str(&self) -> &'static str {
        match self {
            Symbol::Tab => "  ",
            Symbol::Vertical => "│",
            Symbol::Joiner => "├─",
            Symbol::Line => "\n",
        }
    }
}

fn deal_txt_new_line(txt: &mut String, tab_count: u8, is_add: bool) {
    txt.push_str(Symbol::Line.as_str());
    for _i in 0..tab_count {
        txt.push_str(Symbol::Vertical.as_str());
        txt.push_str(Symbol::Tab.as_str());
    }
    txt.push_str(match is_add {
        true => Symbol::Joiner.as_str(),
        false => Symbol::Vertical.as_str(),
    });
}

fn do_gen_txt(
    txt: &mut String,
    relations: &RelationsMap,
    packages: &Option<AHashMap>,
    tab_count: u8,
    max_dep: u8,
) {
    if max_dep == 0 {
        return;
    }
    if let Some(deps) = packages {
        deal_txt_new_line(txt, tab_count, false);
        for (name, version) in deps {
            let children = relations.get(name);
            deal_txt_new_line(txt, tab_count, true);
            txt.push_str(&format!("{} {}", name, version));
            match children {
                Some(rel) => {
                    do_gen_txt(
                        txt,
                        relations,
                        &merge_map(&rel.dependencies, &rel.devDependencies),
                        tab_count + 1,
                        max_dep - 1,
                    );
                }
                None => {}
            }
        }
        deal_txt_new_line(txt, tab_count, false);
    }
}

pub fn gen_txt(relations: &RelationsMap, depth: u8) -> String {
    let root = relations.get("__root__").unwrap();
    let mut txt = String::from(format!("{} {}", root.name, root.version,));
    do_gen_txt(
        &mut txt,
        relations,
        &merge_map(&root.dependencies, &root.devDependencies),
        0,
        depth,
    );

    txt
}
