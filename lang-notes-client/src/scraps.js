export function Space(props) {
    const { spaces } = props
    const spaceHtml = []
    for (let i = 0; i < spaces; i++) {
        spaceHtml.push(<>&nbsp;</>)
    }
    return <>{spaceHtml}</>
}

function TranslationExample(props) {
    const { example, exampleIndex } = props
    return (
        <div>
                <Space spaces={8}/>
                {exampleIndex + 1}. {example.example}
        </div>
    )
}

function TranslationExamples(props) {
    const { examples } = props
    const exampleViews = []
    const set = html => {
        exampleViews.push(html)
    }
    for (let i = 0; i < examples.length; i++) {
        set(<TranslationExample key={i} exampleIndex={i} example={examples[i]}/>)
    }
    return (
        <div>
            { exampleViews }
        </div>
    )
}

function Translation(props) {
    const { translation, translationIndex } = props
    const translationRow = translation.translation
    const { definition } = translationRow
    const { examples, lang } = translation
    return (
        <div>
            <Space spaces={4} />
            {translationIndex + 1}. <span className="gray i">{lang.name}.</span> {definition}
            { examples && examples.length ?
                <TranslationExamples examples={examples} /> : null }
        </div>
    )
}

function Translations(props) {
    const { translations } = props
    const translationViews = []
    const set = html => {
        translationViews.push(html)
    }
    for (let i = 0; i < translations.length; i++) {
        set(<Translation key={i} translationIndex={i} translation={translations[i]} />)
    }
    return (
        <div>
            {translationViews}
        </div>
    )
}

function SearchKey(props) {
    const { searchKey, searchKeyIndex } = props
    const { key } = searchKey
    return (
        <div>
            &nbsp;&nbsp;&nbsp;&nbsp;
            {searchKeyIndex + 1}. {key}
        </div>
    )
}

function SearchKeys(props) {
    const { searchKeys } = props
    const searchKeyViews = []
    const set = html => {
        searchKeyViews.push(html)
    }
    for (let i = 0; i < searchKeys.length; i++) {
        set(<SearchKey key={i} searchKeyIndex={i} searchKey={searchKeys[i]} />)
    }
    return (
        <div>
            {searchKeyViews}
        </div>
    )
}

function DefinitionExample(props) {
    const { example, exampleIndex } = props
    return (
        <div>
                <Space spaces={4} />
                {exampleIndex + 1}. {example.example}
        </div>
    )
}

function DefinitionExamples(props) {
    const { examples } = props
    const exampleViews = []
    const set = html => {
        exampleViews.push(html)
    }
    for (let i = 0; i < examples.length; i++) {
        set(<DefinitionExample key={i} exampleIndex={i} example={examples[i]}/>)
    }
    return (
        <div>
            { exampleViews }
        </div>
    )
}