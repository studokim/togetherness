import React, { useEffect, useState } from 'react'
import "./StatusPage.scss"
import Tr from './Tr'

export function StatusPage(props) {
    const [results, setResults] = useState(null)
    useEffect(() => { }, [])
    return (
        <div className='StatusPage'>
            <div className='name'></div>
            <div className='gold'></div>
            <table>
                <tbody>
                    {results !== null
                        ?
                        results.map(el => <Tr actionId={el.action_id} objectNum={el.object} subjectNum />)

                        :
                        null
                    }
                </tbody>
            </table>
        </div>
    )
}
