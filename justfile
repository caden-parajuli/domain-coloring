default: gen lint

gen:
    flutter pub get
    flutter_rust_bridge_codegen

gen-gh:
    flutter_rust_bridge_codegen ./.bridge_gh_action.yml
dart_output:
  - lib/bridge_generated.dart

lint:
    cd native && cargo fmt
    dart format .

clean:
    flutter clean
    cd native && cargo clean
    
serve *args='':
    flutter pub run flutter_rust_bridge:serve {{args}}
