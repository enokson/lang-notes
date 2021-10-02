import { useEffect, useState } from 'react'
import { useParams, Link } from 'react-router-dom'
import { clusters } from '../examples'

function Space(props) {
    const { spaces } = props
    const spaceHtml = []
    for (let i = 0; i < spaces; i++) {
        spaceHtml.push(<span key={i}>&nbsp;</span>)
    }
    return <>{spaceHtml}</>
}

function TranslationExample(props) {
    const { example, exampleIndex } = props
    return (
        <div className="i gray">
            <Space spaces={4}/>
            {example.example}
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
            <span className="gray i">{lang.name},</span> {definition}
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
    return translationViews
}

function DefinitionExample(props) {
    const { example, exampleIndex } = props
    return (
        <div className="gray i">
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
    return exampleViews
}

function Definition(props) {
    const { definition } = props
    const defRow = definition.definition
    const { examples, translations } = definition
    return (
        <div className="ma2 pa2 ba br3 dib">
            <Link className="link dim blue" to={`/definition/${defRow.id}`}>{defRow.word}</Link>
            { defRow.suffixes ? <span className="gray">&nbsp;{defRow.suffixes}</span>: null }
            { defRow.pronounciation ? <span>&nbsp;[ <span>{defRow.pronounciation}</span> ]</span> : null }<br/>
            {defRow.definition}
            <DefinitionExamples examples={examples || []} />
            <Translations translations={translations || []} />
            {/* <code>{JSON.stringify(defRow)}</code> */}
        </div>
    )
}

function Definitions(props) {
    const { definitions } = props
    const htmlComponents = []
    const set = html => htmlComponents.push(html)
    for (let i = 0; i < definitions.length; i++) {
        set(<Definition key={i} definition={definitions[i]}/>)
    }
    return htmlComponents
}

function Group(props) {
    const { definitions } = props
    return (
        <div>
            <h1>{definitions[0].group.name}</h1>
            <div className="flex items-start flex-wrap">
                <Definitions definitions={definitions}/>
            </div>
        </div>
    )
}

function Groups(props) {
    const { definitions } = props
    const groups = []
    for (let i = 0; i < definitions.length; i++) {
        const groupId = definitions[i].group.id
        let found = false
        for (let groupsI = 0; groupsI < groups.length; groupsI++) {
            if (groupId === groups[groupsI][0].group.id) {
                groups[groupsI].push(definitions[i])
                found = true
                break
            }
        }
        if (!found) {
            groups.push([ definitions[i] ])
        }
    }

    const groupView = []
    for (let i = 0; i < groups.length; i++) {
        groupView.push(<Group key={i} definitions={groups[i]} />)
    }

    return groupView
}

function Cluster() {
    const { id } = useParams()
    const [ cluster, setCluster ] = useState(null)
    useEffect(() => {
        setCluster(clusters[id - 1])
    }, [ cluster, id ])
    return (
        <div className="pa2">
            { cluster ? <Groups definitions={cluster.definitions}/> : null }
            {/* <code>{JSON.stringify(cluster)}</code> */}
        </div>
    )
}

export default Cluster