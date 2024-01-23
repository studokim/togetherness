import React from 'react'

export default function Tr({ actionId, objectNum, subjectNum }) {


    return (
        <tr>
            <td>{actionId}</td>
            <td>{subjectNum}</td>
            <td>{objectNum}</td>
        </tr>
    )
}
