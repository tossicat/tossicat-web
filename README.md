# tossicat for Web

**tossicat for Web**은 임의의 단어와 그 단어에 붙일 조사(즉 토시)를 입력하면, 입력한 조사를 같이 입력한 단어에 자연스러운 형태로 바꿔서 적절하게 변환해 주는 라이브러리를 만드는 리포지토리(Repository)입니다. 변환할 토시가 여러개 들어 있는 문장을 아래와 같은 형식으로 입력하면,

`"{철수, 은} {영희,   과} {밥,  를} 먹습니다."`

다음과 같이 변경해 줍니다.

`"철수는 영희처럼 밥을 먹습니다."`

이것은 `fix_sentence()` 함수가 맡습니다.

```js
function fix_sentence(
    sentence: string, 
    ): string;
```

아래와 같이 사용하시면 됩니다.

```js
fix_sentence("{철수, 은} {영희,   과} {밥,  를} 먹습니다.") // 결과: "철수는 영희처럼 밥을 먹습니다."
```

앞에서처럼 문장을 다루는 것이 아니라 단순하게 특정 단어에 특정 토시를 붙일 때 어떻게 변환해야 하는 것인지를 알고 싶은 경우도 있습니다. 이런 경우에 사용할 함수는 `fix()`입니다.

```js
function fix(
    // 토시를 붙이고자 하는 임의의 단어
    word: string, 
    // 위 단어(word)에 붙일 조사(즉 토시)
    tossi: string
    ): string;
```

아래와 같이 사용하시면 됩니다.

```js
fix("밥","을") // 결과: "밥을" 
fix("철수", "은") // 결과: "철수는"
```

위와 같이 이 두 함수는 임의의 단어와 그 단어에 붙일 조사(즉 토시)를 입력하면 적합한 토시로 변경한 다음 단어와 토시를 합쳐 반환하게 됩니다. 현재 이 프로젝트에서 처리할 수 있는 토시(tossi)는 다음과 같습니다. 지속적으로 추가할 예정입니다.

1. 변환을 고려해야 하는 토시들, 46개
2. 변환할 필요가 없는 토시들, 31개

이 둘을 합쳐서 총 77개의 토시를 처리할 수 있습니다. 여기에 외국어 단어가 입력되었을 경우 그 단어의 발음을 정확하게 알 수 없기 때문에, "(을)를"과 같이 토시를 병기해 변환하는 경우가 일어날 수 있기 때문에 내부적으로 처리할 수 있는 토시 숫자는 훨씬 많습니다. 이와 관련된 자세한 내용은 [RELEASES.md](https://github.com/tossicat/tossicat-core/blob/main/RELEASES.md)를 참고하세요.

이 리포지토리는 Rust로 작성한 [tossicat](https://crates.io/crates/tossicat)을 자바스크립트에서 사용하기 위해 러스트의 wasm-pack을 사용하여 컴파일해 WebAssembly로 변환하고 하는 것을 목표로 하고 있습니다. 현재 이 리포지토리(Repository)를 컴파일해서 [npm](https://www.npmjs.com)에서 [tossicat](https://www.npmjs.com/package/@dialektike/tossicat)이라는 이름이라는 패키지로 배포하고 있습니다. 만약 webpack을 사용하신다면 `package.json`에 "dependencies"에 다음과 같이 패키지를 추가하시면 됩니다.

```json
"dependencies": {
      "@dialektike/tossicat": "^0.6.0"
    },
```

그런 다음 다음과 같이 사용하시면 됩니다.

```js
const js = import("./node_modules/@dialektike/tossicat/tossicat.js");
    js.then(js => {
        temp_1 = js.fix("철수","가");
        temp_2 = js.fix_sentence("{철수, 은} {영희, 처럼} {밥,  를} 먹습니다.");
        temp = temp_1 + temp_2;
        alert(temp);
    });
```

외국어로 병기된 단어가 숫자가 포함된 단어가 입력되더라고 마지막 글자만 한글이거나 숫자이면 적절하게 처리할 수 있습니다. 만약 입력된 단어의 글자수가 많거나 변환할 수 없는 토시가 입력되면 에러가 발생하게 됩니다. 그리고 문장을 변환하고자 하는 경우에도 몇 가지 에러가 발생할 수 있습니다. 에러를 처리하기 위해서는 아래와 같이 하시면 됩니다.

```js
const js = import("./node_modules/@dialektike/tossicat/tossicat.js");
    js.then(js => {
        try {
            const result = js.fix("철수", "apple");
            alert(result);
        } catch (err) {
            console.error(err);
        }
    });
```

에러 메세지에 관한 것은 아래 링크를 참고해주세요.

- [tossicat-core-error.rs](https://github.com/tossicat/tossicat-core/blob/main/src/error.rs)

이 리포지토리에 있는 [test_web 폴더](https://github.com/tossicat/tossicat-web/tree/main/test_web)를 보시면 더 자세히 알 수 있습니다. 이 부분은 [Rust를 WebAssembly로 컴파일하기 - 웹어셈블리 | MDN](https://developer.mozilla.org/ko/docs/WebAssembly/Rust_to_wasm)을 참고했습니다.

## wasm-pack을 사용하여 컴파일하기

wasm-pack을 이용하여 이 리포지토리를 직접 패키지 빌드하는 것은 어렵지 않습니다. 당연히 Rust가 설치되어 있어야 합니다. 그 다음 wasm-pack을 설치해야 합니다. 설치하는 방법은 아래와 같습니다.

```console
cargo install wasm-pack
```

그 다음 다음 명령어로 컴파일하면 됩니다. 주의할 점은 아래 `id` 다신에 자신의 `npm`의 `id`를 넣어야 됩니다.

```console
wasm-pack build --scope id
```

컴파일하고 나면 `pkg` 폴더가 만들어지고 거기에 컴파일된 파일과 관련 파일들이 생기게 됩니다. 그걸 사용하시면 됩니다.

## 참고

이 리포지토리는 Rust 패키지를 위한 것이 아닙니다. wasm-pack을 이용해서 WebAssembly로 컴파일하는 것이 최종 목표입니다. 그래서 `Cargo.toml`도 WebAssembly로 컴파일해 npm에 패키지를 배포하는 것에 맞춰 작성되었습니다. 따라서 이 파일은 Rust로 컴파일하는 것에 적합하지 않습니다. 만약 러스트 코드에 관심이 있으시면, 여기에서 바인딩해서 사용하고 있는 라이브러리인  [tossicat-core](https://github.com/tossicat/tossicat-core)을 참고하세요.
