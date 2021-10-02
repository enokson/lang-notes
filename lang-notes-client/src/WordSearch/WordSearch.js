import { useState } from 'react'
import { Redirect, useHistory, Link } from 'react-router-dom'
import { clusters } from '../examples'
// const Ajv = require("ajv")
// const ajv = new Ajv()

function Definition (props) {
    const { definition } = props
    const defRow = definition.definition
    const { group, translations } = definition
    const [ clicked, setClicked ] = useState(false)
    const history = useHistory()
    const handleClick = e => {

        e.stopPropagation()
    }
    function TranslationLine(props) {
        const { translations } = props
        return translations && translations.length ? (
            <div><span className="gray i">{translations[0].lang.name}</span>. {translations[0].translation.definition}</div>) : 
            null
    }

    return (
        <div className="ma1 pa2 ba dib cf" onClick={handleClick}>
            <div>
                { defRow.prefixes ? <><span className="gray">{defRow.prefixes}-</span>&nbsp;</> : null }
                <Link className="link dim blue" to={`/definition/${defRow.id}`}>{defRow.word}</Link>
                { defRow.suffixes ? <>&nbsp;<span className="gray">{defRow.suffixes}</span></>: null }
            </div>
            <div>
                <span className="i gray">{group.name}</span>.&nbsp;
                {defRow.definition}
            </div>
            <TranslationLine translations={translations}/>
            {/* <div><code>{JSON.stringify(definition)}</code></div> */}
        </div>
    )
}

function Definitions(props) {
    const { definitions } = props
    const definitionViews = []
    const set = html => {
        definitionViews.push(html)
    }
    for (let i = 0; i < definitions.length; i++) {
        set(<Definition key={i} definition={definitions[i]} defIndex={i} />)
    }
    return (
        <div>
            {definitionViews}
            {/* <div><code>{JSON.stringify(definitions)}</code><br/><br/></div> */}
        </div>
    )
}

// function Group(props) {

// }

function Groups(props) {
    const { definitions } = props.definitions
    return (
        <div> Definitions: 
            <code>{ JSON.stringify(definitions, null, 4) }</code>
        </div>
    )
}

function Cluster(props) {
    const { cluster, groupByGroup } = props
    const [ clicked, setClicked ] = useState(false)
    const history = useHistory()
    // history.push('/')
    let html
    if (groupByGroup) {
        html = <Groups />
    } else {
        html = <Definitions definitions={cluster.definitions} />
    }
    const redirect = <Redirect to={{ pathname: `/cluster/${props.cluster.cluster.id}`, push: true, from: '/' }} />
    const handleClick = e => {
        history.push('/')
        setClicked(true)
        e.stopPropagation()
    }
    return (
        <div className="ma2 ba text-center cf" onClick={handleClick}>
            {!clicked ? html : redirect}
            {/* <div><code>{JSON.stringify(cluster)}</code><br/><br/></div> */}
        </div>        
    )
}

function Clusters(props) {
    const { clusters } = props
    const clustersView = []
    const set = html => {
        clustersView.push(html)
    }
    for (let i = 0; i < clusters.length; i++) {
        set(<Cluster key={`${i}`} clusterIndex={i} cluster={clusters[i]} />)
    }
    return (
        <div className="ma2">{ clustersView }</div>
        // <code>{JSON.stringify(clusters)}</code>
    )
}

function WordSearch(props) {
    // const [ keySearch, setKeySearch ] = useState('')
    // const [ translationSearch, setTranslationSearch ] = useState('')
    // const [ searchCluster, setSearchCluster ] = useState(true)
    // const [ groupByGroup, setGroupByGroup ] = useState(false)

    return (
        <div>
            {/* search section */}
            <div>
                {/* return cluster input */}
                <div></div>
                {/* key search */}
                <div></div>
                {/* translation search */}
                <div></div>
            </div>
            <Clusters clusters={clusters}/>
            {/* <br/>
            <code>{ JSON.stringify(clusters) }</code> */}
            {/* results section */}
        </div>
    )

}

export default WordSearch