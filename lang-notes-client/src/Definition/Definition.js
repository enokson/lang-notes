import {
    // Redirect,
    useParams
} from 'react-router-dom'

function Definition() {
    const { id } = useParams()
    return (
        <div>
            definition works! {id}
        </div>
    )
}

export default Definition