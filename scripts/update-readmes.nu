use std assert

export def main [] {
    let old_root_readme = open_readme
    cargo rdme -w mrmx -r README.md --force
    let root_readme = open_readme
    
    let crates = ls -a crates | where type == dir 

    let not_updated = $crates | each {|el|
        cd $el.name
        let old_readme = open_readme
        cargo rdme --force
        let readme = open_readme
        ($readme == $old_readme)
    } | reduce {|a, b| $a and $b } --fold ($root_readme == $old_root_readme)
    assert $not_updated
}

def open_readme [] {
    try { open README.md --raw } catch { '' }
}
