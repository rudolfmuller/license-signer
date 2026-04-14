# license-signer

<b>Simple CLI License Generator written in Rust</b>
<br><br>
POSIX focused - Only (linux)
 - <b>Linux: </b>Supported <br>
 - <b>macOS: </b>Supported (untested)<br>
 - <b>Windows: </b>Untested<br>
<br>

<b>WARNING:</b> This project is still under development, so please help me find any issues (and possible fixes)
<br><br>
<b>HOW TO INSTALL:</b><br> 
Make sure that `~/.cargo/bin` is in your `$PATH`<br>
Install: `$ cargo install license-signer`
<br><br>
<b>HOW TO USE:</b><br>
Add a new license: `$ license-signer add ./MIT.lic`<br>
Remove the license: `$ license-signer remove MIT`<br>
Generate the license: `$ license-signer gen -t MIT --owner="Rudolf Muller"`.


