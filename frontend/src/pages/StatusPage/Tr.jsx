import React from 'react'

export default function Tr({ actionId, objectNum, subjectNum }) {


    return (
        <tr>
            <td>{actionId}</td>
            <td>{objectNum}</td>
            <td>{subjectNum}</td>
        </tr>
    )
}
