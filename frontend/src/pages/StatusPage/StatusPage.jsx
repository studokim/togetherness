import React, { useEffect } from 'react'
import "./StatusPage.scss"
import Tr from './Tr'
import { useSelector } from 'react-redux'

function getStringAction(id) {
    switch (id) {
        case 1: return "Объятий";
        case 2: return "Подслушано";
        case 3: return "Шантаж";
        case 4: return "Слухов пущено";
        case 5: return "Преступления";
        default: return "Неизвестное действие";
    }

}

export function StatusPage() {

    const name = useSelector(state => state.status.name);
    const gold = useSelector(state => state.status.gold);
    const actions = useSelector(state => state.status.actions);
    useEffect(() => { }, []);

    return (
        <div className='StatusPage'>
            <h1 className='name'>{name}</h1>
            <p className='gold'>Вы заработали:</p>
            <p className='gold'>{gold}</p>
            <table className='statusTable' rules="All">
                <tbody>

                    <tr>
                        <td>Action_id</td>
                        <td>Вы</td>
                        <td>Вас</td>
                    </tr>

                    {actions !== null
                        ?
                        actions.map(el => <Tr actionId={getStringAction(el.action_id)} objectNum={el.as_object} subjectNum={el.as_subject} />)
                        :
                        null
                    }

                </tbody>
            </table>
        </div>
    )
}
