# lat

cat for LLMs.

> 현재 브레인스토밍 단계입니다.

lat은 LLM들에게 파일 내용을 '적당히 잘' 보여주기 위한 도구입니다. AGENTS.md 등에 cat 대신에 사용하도록 명시하기를 기대합니다.
여기서 '적당히 잘'은 토큰을 아끼고 LLM 들이 읽기 편한 형태로 바꾸는 것이고, 그 구체적인 방법은 파일의 종류에 따라 다를 것입니다. 그래서 lat 자체의 역할은 확장자를 보고 .lat.json 등의 configuration에 따라 해당 확장자에 해당하는 viewer를 실행해 주는 역할입니다.

```jsonc
// .lat.json
{
  "json": ["json-lat", "$1"],
  "js": ["js-lat", "$1"]
  // ...
}
```

요런 느낌?

## per-확장자 lat의 예시

그렇다면 각 확장자의 viewer는 어떤 역할을 해야할까요? 몇가지 예시를 들어봅시다.

## JSON

우선 보여주기 전에 formatting을 하는게 좋겠죠? 의외로 토큰을 아끼겠다고 minify를 하는것은 LLM들의 독해에 악영향을 주어서 좋지 않고, indent를 해주는게 좋습니다.
또한 내용중에 일일이 다 읽을 필요가 없는 매우 큰 숫자 배열 등이 포함되어 있는 경우를 생각해봅시다. 이럴때 해당 부분을 링크로 대체해서 보여줄수 있겠죠.

```bash
> cat big.json:data.a_big_array
{
  "data": {
    "a_big_array": [ 0.8326, 0.1345, 0.744, ... ],
```

대신에 lat을 사용하면,

```bash
> lat big.json
{
  "data": {
    "a_big_array": {:data.a_big_array},
```

로 보여줍니다. 혹시 LLM이 `data.a_big_array`의 내용을 궁금해 한다면, `{:`, `:}`로 감싸져있는 링크를 명시에서 읽을 수 있습니다.

```bash
> lat big.json:data.a_big_array
[ 0.8326, 0.1345, 0.744, ... ]
```

## JS

실제로 lat을 쓰게될 클로드와 GPT에게 물어보니, 소스 코드를 읽을때는 맨 앞에 Overview가 있으면 도움이 된다고 합니다. 본인이 그렇다는데 맞겠죠?


## AsciiDoc, reStructuredText, djot 등등

Markdown과 같은 경량 마크업 언어입니다. LLM이 마크다운을 좋아하는 듯하니, pandoc으로 Markdown으로 변경해서 보여주면 좀더 잘 읽지 않을까요?
