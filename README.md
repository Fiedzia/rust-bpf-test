Trying to get bpf probing working with rust.
I am using rust nightly, ubuntu 16.10

Steps:

1. snap install --devmode bcc
2. cargo run
3. (as root):

    #that works
    bcc.trace -p `pidof probes` 'u::loop'

    #that doesn't
    bcc.trace -p `pidof probes` 'u::loop "%d", arg0' 

