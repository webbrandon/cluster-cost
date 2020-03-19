pub struct CompletionScript { }

    impl CompletionScript {
        pub fn bash() {
            println!("{}",r#"
    _cluster-cost() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            cluster-cost)
                cmd="cluster-cost"
                ;;
            
            analyze)
                cmd+="__analyze"
                ;;
            bash)
                cmd+="__bash"
                ;;
            completions)
                cmd+="__completions"
                ;;
            configuration)
                cmd+="__configuration"
                ;;
            elvish)
                cmd+="__elvish"
                ;;
            fish)
                cmd+="__fish"
                ;;
            help)
                cmd+="__help"
                ;;
            powershell)
                cmd+="__powershell"
                ;;
            predict)
                cmd+="__predict"
                ;;
            set)
                cmd+="__set"
                ;;
            zsh)
                cmd+="__zsh"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        cluster-cost)
            opts=" -d -v -h  --daemon --dry-run --verbose --help --port --host   analyze predict configuration help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        cluster__cost__analyze)
            opts=" -v -h -V -f -n -c  --dry-run --verbose --help --version --filter --namespace --context --eks  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --filter)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --namespace)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --context)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --eks)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration)
            opts=" -h -V  --help --version   set completions help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration__completions)
            opts=" -h -V  --help --version   bash fish zsh powershell elvish help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration__completions__bash)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration__completions__elvish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration__completions__fish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration__completions__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration__completions__powershell)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration__completions__zsh)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__configuration__set)
            opts=" -h -V  --help --version  <action> <node-type> <price> <cpu> <memory> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        cluster__cost__predict)
            opts=" -h -V -n -c -m -s  --help --version --node-type --cpu --memory --scale  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --node-type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -n)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cpu)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --memory)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --scale)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _cluster-cost -o bashdefault -o default cluster-cost

    "#);
        }

        pub fn fish() {
            println!("{}",r#"
    complete -c cluster-cost -n "__fish_use_subcommand" -l port -d 'Daemon mode port'
complete -c cluster-cost -n "__fish_use_subcommand" -l host -d 'Daemon mode host'
complete -c cluster-cost -n "__fish_use_subcommand" -s d -l daemon -d 'Daemon mode'
complete -c cluster-cost -n "__fish_use_subcommand" -l dry-run -d 'Don\'t run commands only log'
complete -c cluster-cost -n "__fish_use_subcommand" -s v -l verbose -d 'Enable verbose logging'
complete -c cluster-cost -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_use_subcommand" -f -a "analyze" -d 'Analyze cluster cost'
complete -c cluster-cost -n "__fish_use_subcommand" -f -a "predict" -d 'Predict cost'
complete -c cluster-cost -n "__fish_use_subcommand" -f -a "configuration" -d 'Configuration options'
complete -c cluster-cost -n "__fish_use_subcommand" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c cluster-cost -n "__fish_seen_subcommand_from analyze" -s f -l filter -d 'Filter container list. ( ie: my-deployment-name )'
complete -c cluster-cost -n "__fish_seen_subcommand_from analyze" -s n -l namespace -d 'Namespace target. ( ie: Environment )'
complete -c cluster-cost -n "__fish_seen_subcommand_from analyze" -s c -l context -d 'Cluster target'
complete -c cluster-cost -n "__fish_seen_subcommand_from analyze" -l eks -d 'Update token for eks using aws profile'
complete -c cluster-cost -n "__fish_seen_subcommand_from analyze" -l dry-run -d 'Don\'t run commands only log'
complete -c cluster-cost -n "__fish_seen_subcommand_from analyze" -s v -l verbose -d 'Enable verbose logging'
complete -c cluster-cost -n "__fish_seen_subcommand_from analyze" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from analyze" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from predict" -s n -l node-type -d 'The node type used for calculation'
complete -c cluster-cost -n "__fish_seen_subcommand_from predict" -s c -l cpu -d 'CPU requirement'
complete -c cluster-cost -n "__fish_seen_subcommand_from predict" -s m -l memory -d 'Cluster target'
complete -c cluster-cost -n "__fish_seen_subcommand_from predict" -s s -l scale -d 'Cluster target'
complete -c cluster-cost -n "__fish_seen_subcommand_from predict" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from predict" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from configuration" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from configuration" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from configuration" -f -a "set" -d 'Set configuration file value to something new'
complete -c cluster-cost -n "__fish_seen_subcommand_from configuration" -f -a "completions" -d 'Completion scripts for various shells'
complete -c cluster-cost -n "__fish_seen_subcommand_from configuration" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c cluster-cost -n "__fish_seen_subcommand_from set" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from set" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from completions" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from completions" -f -a "bash" -d 'Bash completion script'
complete -c cluster-cost -n "__fish_seen_subcommand_from completions" -f -a "fish" -d 'Fish completion script'
complete -c cluster-cost -n "__fish_seen_subcommand_from completions" -f -a "zsh" -d 'Zsh completion script'
complete -c cluster-cost -n "__fish_seen_subcommand_from completions" -f -a "powershell" -d 'PowerShell completion script'
complete -c cluster-cost -n "__fish_seen_subcommand_from completions" -f -a "elvish" -d 'Elvish completion script'
complete -c cluster-cost -n "__fish_seen_subcommand_from completions" -f -a "help" -d 'Prints this message or the help of the given subcommand(s)'
complete -c cluster-cost -n "__fish_seen_subcommand_from bash" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from bash" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from fish" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from fish" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from zsh" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from zsh" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from powershell" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from powershell" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from elvish" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from elvish" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'
complete -c cluster-cost -n "__fish_seen_subcommand_from help" -s h -l help -d 'Prints help information'
complete -c cluster-cost -n "__fish_seen_subcommand_from help" -s V -l version -d 'Prints version information'

    "#);
        }

        pub fn zsh() {
            println!("{}",r#"
    #compdef cluster-cost

autoload -U is-at-least

_cluster-cost() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'--port=[Daemon mode port]' \
'--host=[Daemon mode host]' \
'-d[Daemon mode]' \
'--daemon[Daemon mode]' \
'--dry-run[Don'\''t run commands only log]' \
'-v[Enable verbose logging]' \
'--verbose[Enable verbose logging]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
":: :_cluster-cost_commands" \
"*::: :->cluster-cost" \
&& ret=0
    case $state in
    (cluster-cost)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:cluster-cost-command-$line[1]:"
        case $line[1] in
            (analyze)
_arguments "${_arguments_options[@]}" \
'-f+[Filter container list. ( ie: my-deployment-name )]' \
'--filter=[Filter container list. ( ie: my-deployment-name )]' \
'-n+[Namespace target. ( ie: Environment )]' \
'--namespace=[Namespace target. ( ie: Environment )]' \
'-c+[Cluster target]' \
'--context=[Cluster target]' \
'--eks=[Update token for eks using aws profile]' \
'--dry-run[Don'\''t run commands only log]' \
'-v[Enable verbose logging]' \
'--verbose[Enable verbose logging]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(predict)
_arguments "${_arguments_options[@]}" \
'-n+[The node type used for calculation]' \
'--node-type=[The node type used for calculation]' \
'-c+[CPU requirement]' \
'--cpu=[CPU requirement]' \
'-m+[Cluster target]' \
'--memory=[Cluster target]' \
'-s+[Cluster target]' \
'--scale=[Cluster target]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(configuration)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_cluster-cost__configuration_commands" \
"*::: :->configuration" \
&& ret=0
case $state in
    (configuration)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:cluster-cost-configuration-command-$line[1]:"
        case $line[1] in
            (set)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::action -- Perform action on node type config. (create, read, update, delete):_files' \
'::node-type -- Field being targeted for change:_files' \
'::price -- Target field new value:_files' \
'::cpu -- Target field new value:_files' \
'::memory -- Target field new value:_files' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_cluster-cost__configuration__completions_commands" \
"*::: :->completions" \
&& ret=0
case $state in
    (completions)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:cluster-cost-configuration-completions-command-$line[1]:"
        case $line[1] in
            (bash)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Bash completion script:_files' \
&& ret=0
;;
(fish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Fish completion script:_files' \
&& ret=0
;;
(zsh)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Zsh completion script:_files' \
&& ret=0
;;
(powershell)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- PowerShell completion script:_files' \
&& ret=0
;;
(elvish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Elvish completion script:_files' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
;;
(help)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
        esac
    ;;
esac
}

(( $+functions[_cluster-cost_commands] )) ||
_cluster-cost_commands() {
    local commands; commands=(
        "analyze:Analyze cluster cost" \
"predict:Predict cost" \
"configuration:Configuration options" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'cluster-cost commands' commands "$@"
}
(( $+functions[_cluster-cost__analyze_commands] )) ||
_cluster-cost__analyze_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost analyze commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration__completions__bash_commands] )) ||
_cluster-cost__configuration__completions__bash_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost configuration completions bash commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration__completions_commands] )) ||
_cluster-cost__configuration__completions_commands() {
    local commands; commands=(
        "bash:Bash completion script" \
"fish:Fish completion script" \
"zsh:Zsh completion script" \
"powershell:PowerShell completion script" \
"elvish:Elvish completion script" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'cluster-cost configuration completions commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration_commands] )) ||
_cluster-cost__configuration_commands() {
    local commands; commands=(
        "set:Set configuration file value to something new" \
"completions:Completion scripts for various shells" \
"help:Prints this message or the help of the given subcommand(s)" \
    )
    _describe -t commands 'cluster-cost configuration commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration__completions__elvish_commands] )) ||
_cluster-cost__configuration__completions__elvish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost configuration completions elvish commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration__completions__fish_commands] )) ||
_cluster-cost__configuration__completions__fish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost configuration completions fish commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration__completions__help_commands] )) ||
_cluster-cost__configuration__completions__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost configuration completions help commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration__help_commands] )) ||
_cluster-cost__configuration__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost configuration help commands' commands "$@"
}
(( $+functions[_cluster-cost__help_commands] )) ||
_cluster-cost__help_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost help commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration__completions__powershell_commands] )) ||
_cluster-cost__configuration__completions__powershell_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost configuration completions powershell commands' commands "$@"
}
(( $+functions[_cluster-cost__predict_commands] )) ||
_cluster-cost__predict_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost predict commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration__set_commands] )) ||
_cluster-cost__configuration__set_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost configuration set commands' commands "$@"
}
(( $+functions[_cluster-cost__configuration__completions__zsh_commands] )) ||
_cluster-cost__configuration__completions__zsh_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'cluster-cost configuration completions zsh commands' commands "$@"
}

_cluster-cost "$@"
    "#);
        }

        pub fn powershell() {
            println!("{}",r#"
    
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'cluster-cost' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'cluster-cost'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'cluster-cost' {
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'Daemon mode port')
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'Daemon mode host')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Daemon mode')
            [CompletionResult]::new('--daemon', 'daemon', [CompletionResultType]::ParameterName, 'Daemon mode')
            [CompletionResult]::new('--dry-run', 'dry-run', [CompletionResultType]::ParameterName, 'Don''t run commands only log')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('analyze', 'analyze', [CompletionResultType]::ParameterValue, 'Analyze cluster cost')
            [CompletionResult]::new('predict', 'predict', [CompletionResultType]::ParameterValue, 'Predict cost')
            [CompletionResult]::new('configuration', 'configuration', [CompletionResultType]::ParameterValue, 'Configuration options')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'cluster-cost;analyze' {
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Filter container list. ( ie: my-deployment-name )')
            [CompletionResult]::new('--filter', 'filter', [CompletionResultType]::ParameterName, 'Filter container list. ( ie: my-deployment-name )')
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'Namespace target. ( ie: Environment )')
            [CompletionResult]::new('--namespace', 'namespace', [CompletionResultType]::ParameterName, 'Namespace target. ( ie: Environment )')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Cluster target')
            [CompletionResult]::new('--context', 'context', [CompletionResultType]::ParameterName, 'Cluster target')
            [CompletionResult]::new('--eks', 'eks', [CompletionResultType]::ParameterName, 'Update token for eks using aws profile')
            [CompletionResult]::new('--dry-run', 'dry-run', [CompletionResultType]::ParameterName, 'Don''t run commands only log')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Enable verbose logging')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;predict' {
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'The node type used for calculation')
            [CompletionResult]::new('--node-type', 'node-type', [CompletionResultType]::ParameterName, 'The node type used for calculation')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'CPU requirement')
            [CompletionResult]::new('--cpu', 'cpu', [CompletionResultType]::ParameterName, 'CPU requirement')
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, 'Cluster target')
            [CompletionResult]::new('--memory', 'memory', [CompletionResultType]::ParameterName, 'Cluster target')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Cluster target')
            [CompletionResult]::new('--scale', 'scale', [CompletionResultType]::ParameterName, 'Cluster target')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;configuration' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('set', 'set', [CompletionResultType]::ParameterValue, 'Set configuration file value to something new')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Completion scripts for various shells')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'cluster-cost;configuration;set' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;configuration;completions' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('bash', 'bash', [CompletionResultType]::ParameterValue, 'Bash completion script')
            [CompletionResult]::new('fish', 'fish', [CompletionResultType]::ParameterValue, 'Fish completion script')
            [CompletionResult]::new('zsh', 'zsh', [CompletionResultType]::ParameterValue, 'Zsh completion script')
            [CompletionResult]::new('powershell', 'powershell', [CompletionResultType]::ParameterValue, 'PowerShell completion script')
            [CompletionResult]::new('elvish', 'elvish', [CompletionResultType]::ParameterValue, 'Elvish completion script')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Prints this message or the help of the given subcommand(s)')
            break
        }
        'cluster-cost;configuration;completions;bash' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;configuration;completions;fish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;configuration;completions;zsh' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;configuration;completions;powershell' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;configuration;completions;elvish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;configuration;completions;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;configuration;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'cluster-cost;help' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

    "#);
        }

        pub fn elvish() {
            println!("{}",r#"
            
edit:completion:arg-completer[cluster-cost] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'cluster-cost'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'cluster-cost'= {
            cand --port 'Daemon mode port'
            cand --host 'Daemon mode host'
            cand -d 'Daemon mode'
            cand --daemon 'Daemon mode'
            cand --dry-run 'Don''t run commands only log'
            cand -v 'Enable verbose logging'
            cand --verbose 'Enable verbose logging'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand analyze 'Analyze cluster cost'
            cand predict 'Predict cost'
            cand configuration 'Configuration options'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'cluster-cost;analyze'= {
            cand -f 'Filter container list. ( ie: my-deployment-name )'
            cand --filter 'Filter container list. ( ie: my-deployment-name )'
            cand -n 'Namespace target. ( ie: Environment )'
            cand --namespace 'Namespace target. ( ie: Environment )'
            cand -c 'Cluster target'
            cand --context 'Cluster target'
            cand --eks 'Update token for eks using aws profile'
            cand --dry-run 'Don''t run commands only log'
            cand -v 'Enable verbose logging'
            cand --verbose 'Enable verbose logging'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;predict'= {
            cand -n 'The node type used for calculation'
            cand --node-type 'The node type used for calculation'
            cand -c 'CPU requirement'
            cand --cpu 'CPU requirement'
            cand -m 'Cluster target'
            cand --memory 'Cluster target'
            cand -s 'Cluster target'
            cand --scale 'Cluster target'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;configuration'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand set 'Set configuration file value to something new'
            cand completions 'Completion scripts for various shells'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'cluster-cost;configuration;set'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;configuration;completions'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand bash 'Bash completion script'
            cand fish 'Fish completion script'
            cand zsh 'Zsh completion script'
            cand powershell 'PowerShell completion script'
            cand elvish 'Elvish completion script'
            cand help 'Prints this message or the help of the given subcommand(s)'
        }
        &'cluster-cost;configuration;completions;bash'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;configuration;completions;fish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;configuration;completions;zsh'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;configuration;completions;powershell'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;configuration;completions;elvish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;configuration;completions;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;configuration;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'cluster-cost;help'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}

    "#);
        }
    }
    