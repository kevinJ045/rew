

fn main() {
  println!(
    "{}",
    rew_jsx::compile_jsx(
      String::from(
        r"#
    let something = <>
      <div>
        <SomeElement value={hello} />
        <MyElt />
        <myelt />
        <myelt prop={<b n={u}></b>}>
          {sm.map -> <b>{i}</b>}
        </myelt>
      </div>
    </>
  #"
      ),
      Some("jsx".to_string())
    )
  );
}
