import React from 'react'
import "./StatusPage.scss"
import Tr from './Tr'
import { useSelector } from 'react-redux'

function getStringAction(id) {
    switch (id) {
        case 1: return "Объятий";
        case 2: return "Краж";
        case 3: return "Шантаж";
        case 4: return "Подкупов";
        case 5: return "Лоббирований";
        default: return "Неизвестное действие";
    }
}
function getGoldWord(gold) {

    if (gold >= 5 && gold <= 20) return "Золотых монет";
    const tempGold = gold % 10;
    switch (tempGold) {
        case 1:
            if (gold !== 11) return "Золотая монета";
        case 2:
            if (gold !== 11) return "Золотые монеты";
        case 3: return "Золотые монеты";
        case 4: return "Золотые монеты";
        case 5: return "Золотыx монет";
        case 6: return "Золотыx монет";
        case 7: return "Золотыx монет";
        case 8: return "Золотыx монет";
        case 9: return "Золотыx монет";
        case 0: return "Золотыx монет";
        default: return "Золотыx монет";
    }
}
export function StatusPage() {

    const name = useSelector(state => state.status.name);
    const gold = useSelector(state => state.status.gold);
    const actions = useSelector(state => state.status.actions);


    return (
        <div className='StatusPage'>
            <h1 className='name'>{name}</h1>
            <div className="StatusPageInner">
                <div>
                    <p className='gold'>Вы заработали:</p>
                    <p className='gold'>{gold + " " + getGoldWord(gold)}</p>
                </div>
                <table className='statusTable' rules="All">
                    <tbody>

                        <tr>
                            <td>Взаимодействия</td>
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
        </div>
    )
}
