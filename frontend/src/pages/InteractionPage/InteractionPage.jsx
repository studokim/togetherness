import React, { useEffect, useState } from 'react';
import "./InteractionPage.scss"
import CustomButton from '../../UI/CustomButton/CustomButton';
import { useDispatch, useSelector } from 'react-redux';
import { actionEnabled, createAction, getPerson } from '../../redux/status';
import MessageSimple from '../MainPage/Message/MessageSimple';

const InteractionPage = ({ targetId, close, id }) => {

    const messageAboutStart = useSelector(state => state.status.messageAboutStart);
    const [interractionEnebled, setInterractionEnebled] = useState(null);
    const fractionImg = useSelector(state => state.status.fractionsImgs);
    const myFractionId = useSelector(state => state.status.fraction);
    const avatars = useSelector(state => state.status.avatars);
    const [notEnoughGold, setNotEnoughGold] = useState(false);

    const [target, setTarget] = useState({ name: null, avatarId: null, targetId: targetId, fractionId: null });
    const dispatch = useDispatch();

    useEffect(() => {

        if (targetId !== null) {
            dispatch(getPerson({
                id: targetId, callback: (name, uuuuu, avatarId, fractionId) => {//при получении данных 
                    setTarget({ name: name, avatarId: avatarId, targetId: targetId, fractionId: fractionId });
                    dispatch(actionEnabled({ targetId: targetId, callback: (val) => setInterractionEnebled(val) }));
                },
                errorHandler: () => { }
            }));
        }
        console.log("targetId", targetId)
    }, [targetId])


    useEffect(() => { if (setNotEnoughGold) setTimeout(() => { setNotEnoughGold(false) }, 3000) }, [notEnoughGold]) //Выключение сообщения о нехватке денег в течении 3 сек


    function action(actionId) {
        dispatch(createAction({
            targetId, actionId,
            callback: () => { close() },
            errorHandler: () => { console.log("Попали в errorHandler по notEnoughGold"); setNotEnoughGold(true) }
        }));
    }
    console.log("target.fractionId, ", target.fractionId);
    console.log("myFractionId, ", myFractionId);
    console.log("Number(target.fractionId) - 1, ", Number(target.fractionId) - 1);
    console.log("fractionImg[Number(target.fractionId) - 1], ", fractionImg[Number(target.fractionId) - 1]);

    return (
        <div className='InteractionPage'>
            <div className='InteractionPageInner'>

                {notEnoughGold ? <MessageSimple message="У вас недостаточно золота" close={() => { setNotEnoughGold(false); }} /> : null}
                {messageAboutStart === 1 ? <MessageSimple message="Ночь интриг завершилась" close={() => { close() }} /> : null}

                {/* <div>{targetId}</div> */}
                <>
                    {/* ________________ ИМЯ ЦЕЛИ ДЛЯ ВЗАИМОДЕЙСТВИЯ ____________________ */}
                    <div className='targetName'>{target.name}</div>
                    {/* ________________ АВАТАР С ФРАКИЦЕЙ ____________________ */}
                    <div className='imgContainer'>
                        <img className='targetAvatar' src={target.avatarId !== null ? avatars[target.avatarId] : `/images/unicorn.jpg`} />

                        {target.fractionId === myFractionId
                            ?
                            <div className='fractionImg'>
                                <img src={`${fractionImg[Number(target.fractionId) - 1]}`} />
                            </div>
                            :
                            null}
                    </div>
                </>
                {/* ________________ ВЗАИМОДЕЙСТВИЯ ____________________ */}
                {interractionEnebled === null //еще не загрузились данные 
                    ?
                    null
                    :
                    interractionEnebled === true //можно взаимодействовать 
                        ?
                        <div className='interractions'>
                            <CustomButton onClick={() => { action(1) }}>Обнять +1/+1</CustomButton>
                            <CustomButton onClick={() => { action(2) }}>Обокрасть +2/-2</CustomButton>
                            <CustomButton onClick={() => { action(3) }}>Шантажировать -1/-4</CustomButton>
                            <CustomButton onClick={() => { action(4) }}>Подкупить -1/+3</CustomButton>
                            <CustomButton onClick={() => { action(5) }}>Лоббировать 0/+2</CustomButton>
                            <CustomButton onClick={() => { close() }}>Уйти</CustomButton>
                        </div>
                        : //нельзя взаимодействовать
                        <>
                            <h1>Больше ничего нельзя сделать</h1>
                            <CustomButton onClick={() => { close() }}>Уйти</CustomButton>
                        </>
                }
            </div>
        </div >
    );
}

export default InteractionPage;
