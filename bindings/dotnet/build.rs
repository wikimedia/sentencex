fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("sentencex_dotnet")
        .csharp_dll_name_if("IOS", "__Internal")
        .csharp_namespace("Sentencex.Native")
        .generate_csharp_file("Sentencex/NativeMethods.g.cs")
        .unwrap();
}
