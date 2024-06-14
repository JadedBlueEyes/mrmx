use std assert

export def main [] {
    let old_root_readme = open README.md --raw
    cargo rdme -w mrmx -r README.md --force
    let root_readme = open README.md --raw
    
    let crates = ls -a crates | where type == dir 

    let not_updated = $crates | each {|el|
        cd $el.name
        let old_readme = open README.md --raw
        cargo rdme --force
        let readme = open README.md --raw
        ($readme == $old_readme)
    } | reduce {|a, b| $a and $b } --fold ($root_readme == $old_root_readme)
    assert $not_updated
}
