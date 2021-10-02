import './NewWord.css';
import { useState } from 'react'
const Ajv = require("ajv")
const ajv = new Ajv()

const translation = {
  type: 'object',
  properties: {
      lang: { type: 'string' },
      value: { type: 'string' }
  }
}

const definition = {
  type: 'object',
  properties: {
      value: { type: 'string' },
      translations: { type: 'array', items: translation },
      group: { type: 'string' },
      pronounciation: { type: 'string' },
      example: { type: 'string' },
      search: { type: 'array', items: { type: 'string' } }
  }
}

const defMetaDataView = {
  type: 'object',
  properties: {
    mapKey: { type: 'string' },
    def: definition
  }
}

const schemaGroupView = {
  type: 'object',
  properties: {
    name: { type: 'string' },
    defs: { type: 'array', items: defMetaDataView }
  }
}

const schemaGroupsView = {
  type: 'array',
  items: schemaGroupView
}


function clone(v) {
  return JSON.parse(JSON.stringify(v))
}

function DefinitionItem (props) {
  let spaces = []
  for (let count = 1; count <= props.order.toString().length; count++) {
    spaces.push(<span>&nbsp;</span>)
  }
  return (
    <li key={`list-item-${props.order}`} className="ph3 pv3 bb b--light-silver">
      <div key={`value-${props.order}`}>{props.order}. {props.def.value}</div>
      <div key={`example-${props.order}`}>{spaces}&nbsp;&nbsp;&nbsp;{props.def.example}</div>
    </li>
  )
}

function DefinitionList (props) {
  const group = props.group;
  const dataValid = ajv.validate(schemaGroupView, group)
  if (!dataValid) {
    console.error(`invalid data received in DefinitionList`)
    return
  }
  const listItems = []
  const defKeys = Object.keys(group.defs)
  for (let key of defKeys) {
    const def = group.defs[key]
    listItems.push(<DefinitionItem key={key} order={key} def={def}/>)
  }
  return (
    <article key={`${group.name}`}  className="">
      <h1 className="f4 bold center mw6">{group.name}</h1>
      <ul key={`list`} className="list pl0 ml0 center mw6 ba b--light-silver br2">
        { listItems }
      </ul>
    </article>
  )
}

function Lists (props) {
  const valid = ajv.validate(schemaGroupsView, props.groupLists)
  if (!valid) {
    console.error(`received invalid data in Lists fn`)
    throw new Error(`received invalid data in Lists fn`)
  }
  const groups = clone(props.groupLists)
  

}

function toMap (arr) {
  const obj = {}
  for (let i = 0; i < arr.length; i++) {
    obj[i] = clone(arr[i])
  }
  return obj
}

function toArray(defs) {
  const o = clone(defs)
  const groups = {}
  const groupArr = []
  const keys = Object.keys(o)
  if (!keys.length) {
    return groupArr
  }
  for (let key of keys) {
    const def = o[key]
    if (!groups[def.group]) {
      groups[def.group] = { name: def.group, defs: [ clone(def) ] }
    } else {
      groups[def.group].defs.push(clone(def))
    }
  }
  const groupKeys = Object.keys(groups)
  for (let key of groupKeys) {
    groupArr.push(clone(groups[key]))
  }
  return groupArr
}

function NewWord() {
  const [ word, setWord ] = useState('')
  const [ pronounciation, setPronounciation ] = useState('')
  const [ definitions, setDefinitions ] = useState([])
  const [ defMap, setDefMap ] = useState(toMap([
    { value: 'рафтан', def: 'infinitve form', group: 'verb', pronounciation: '', example: '', search: [ 'рафтан' ], translations: [ { lang: 'eng', value: 'to run' } ],  }
  ]))
  const [ definitionForm, setDefinitionForm ] = useState({ value: '', group: '', example: '' })

  // const definitionItems = []

  
  // for (let groupName of groups) {
  //   definitionItems.push(
  //     <DefinitionList key={groupIndex} groups={definitionGroups} group={groupName} order={displayOrder} />
  //   )
  // }

  const handleClick = (id, value) => {
    if (id === 'definitionForm.save') {
      // const def = JSON.parse(JSON.stringify(value))
      // setDefinitionForm(prev => ({ ...prev, ...{ value: '', example: '' } }))
      // const newDefinitionList = addItemToList(definitions, def)
      // setDefinitions(newDefinitionList)
    }
  }

  const handleInput = (id, value) => {
    // if (id === 'definitionForm.input') {
    //   console.log(value)
    //   setDefinitionForm(prev => ({ ...prev, ...value }))
    // } else if (id === 'definitionForm.group') {
    //   // setDefinitionForm({ group: value })
    // } else if (id === 'definitionForm.example') {
    //   // setDefinitionForm({ example: value })
    // }
  }

  return (
    <div className="NewWord flex justify-center">
      <div className="w-50">
        <div className="w-100 pa4 black-80">
          <label htmlFor="word" className="f6 b db mb2">Word</label>
          <input id="word" className="input-reset ba b--black-20 pa2 mb2 db w-100" type="text" aria-describedby="word-desc" />
          <small id="word-desc" className="f6 black-60 db mb2">The word itself</small>

          <label htmlFor="prnounciation" className="f6 b db mb2">Pronounciation <span className="normal black-60">(optional)</span></label>
          <input id="prnounciation" className="input-reset ba b--black-20 pa2 mb2 db w-100" type="text" aria-describedby="prnounciation-desc" />
          <small id="prnounciation-desc" className="f6 black-60 db mb2">The pronounciation in IPA</small>
        </div>

        <div className="w-100 pa4 black-80">
          <label htmlFor="definition" className="f6 b db mb2">Definition</label>
          <input 
            id="definition" 
            className="input-reset ba b--black-20 pa2 mb2 db w-100" 
            type="text" 
            aria-describedby="definition-desc" 
            placeholder="Definition: ran, the past tense variant of run."
            value={definitionForm.value}
            onChange={(e) => handleInput('definitionForm.input', { value: e.target.value })}/>

          <label htmlFor="definitionGroup" className="f6 b db mb2">Group</label>
          <input 
            id="definitionGroup" 
            className="input-reset ba b--black-20 pa2 mb2 db w-100" 
            type="text" aria-describedby="definitionGroup-desc" 
            placeholder="Group (verb, noun, adjective, etc.)"
            value={definitionForm.group}
            onChange={(e) => handleInput('definitionForm.input', { group: e.target.value })}/>

          <label htmlFor="definitionExample" className="f6 b db mb2">Example <span className="normal black-60">(optional)</span></label>
          <input 
            id="definitionExample" 
            className="input-reset ba b--black-20 pa2 mb2 db w-100" 
            type="text" 
            aria-describedby="definitionExample-desc"
            value={definitionForm.example}
            onChange={(e) => handleInput('definitionForm.input', { example: e.target.value })}/>
          <a 
            className="w-100 f6 link dim br3 ph3 pv2 mb2 dib white bg-black text-center" 
            href="#0"
            onClick={() => handleClick('definitionForm.save', definitionForm)}>Save Definition</a>
        </div>

        <div className="w-100 pa4 black-80">
          <label htmlFor="synonym" className="f6 b db mb2">Synonym</label>
          <input id="synonym" className="input-reset ba b--black-20 pa2 mb2 db w-100" type="text" aria-describedby="synonym-desc" placeholder="Synonym" />
          <a className="w-100 f6 link dim br3 ph3 pv2 mb2 dib white bg-black text-center" href="#0">Save Synonym</a>
        </div>
      </div>

      <div className="w-50"> 
        <Lists groupLists={toArray(defMap)} />
      </div>
    </div>
  );
}

export default NewWord;