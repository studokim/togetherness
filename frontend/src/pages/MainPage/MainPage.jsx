import React, { useEffect, useState } from 'react'
import "./MainPage.scss"
import CustomButton from '../../UI/CustomButton/CustomButton'
import { useNavigate } from 'react-router-dom';

import { QRCodeSVG } from 'qrcode.react';

import { QrScanner } from '@yudiel/react-qr-scanner';
import { useDispatch, useSelector } from 'react-redux';
import InteractionPage from '../InteractionPage/InteractionPage';
import { Timer } from './Timer/Timer';
import { useGold } from './useGold';
import { getObjectActions, getSubjectActions, setObjectActions, setSubjectActions, getActions, setActions } from '../../redux/status';
import Message from './Message/Message';


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

export default function MainPage() {

    const [visibility, setVisibility] = useState({
        qrCode: false,
        qrScaner: false,
        message: false,
        // ineraction: false,
    })
    const id = useSelector((state) => state.status.id);
    const name = useSelector((state) => state.status.name);
    const avatars = useSelector((state) => state.status.avatars);
    const selectedAvatar = useSelector((state) => state.status.selectedAvatar);
    const messageAboutStart = useSelector(state => state.status.messageAboutStart);
    const [targetId, setTargetId] = useState(null);
    // const [messageVisible, setMessageVisible] = useState(false);
    const gold = useGold();
    const dispatch = useDispatch();
    const navigator = useNavigate();
    console.log(messageAboutStart)
    useEffect(() => {
        console.log("ID", id)
    }, [id])

    useEffect(() => {
        console.log("visibility", visibility)
    }, [visibility])


    return (
        <div className='MainPage'>
            {//_______ ОТОБРАЖАЕМ ЛИБО СТРАНИЦУ ВЗАИМОДЕЙСТВИЯ ЛИБО ГЛАВНУЮ СТРАНИЦУ _________//
                targetId !== null   //если получили id другого игроока                
                    ?
                    <InteractionPage
                        targetId={targetId}
                        close={() => { setTargetId(null); setVisibility({ qrCode: false, qrScaner: false }) }}
                        id={id}
                    />
                    :
                    <>
                        {
                            visibility.qrCode ?
                                <div className='qr-code-substrate' onClick={(e) => { e.stopPropagation(); setVisibility({ ...visibility, qrCode: false }); }}>
                                    <div className='qr-code-substrate2'>
                                        <QRCodeSVG value={id} size={256} />
                                    </div>
                                </div>
                                :
                                null
                        }

                        {
                            visibility.qrScaner
                                ?
                                <div className='qr-code-substrate' onClick={(e) => { e.stopPropagation(); setVisibility({ ...visibility, qrScaner: false }); }}>
                                    <QrScanner
                                        containerStyle={{ position: "absolute", height: "100%", width: "100%" }}
                                        onDecode={(result) => { console.log(result); setTargetId(result); }}
                                        onError={(error) => { console.log(error?.message); console.log(error); setTargetId("SCANER ERROR"); }}
                                    />
                                </div>
                                :
                                null
                        }

                        {
                            visibility.message
                                ?
                                <Message message={messageAboutStart} close={() => setVisibility({ ...visibility, message: false })} />
                                :
                                null
                        }

                        <Timer />

                        <h1>{name}</h1>

                        <div className='avatar'>
                            {
                                selectedAvatar === null ?
                                    <div className='avatarImage'><img src={`images/loader.svg`} /></div> :
                                    <img className='avatarImage' src={`${avatars[selectedAvatar]}`} />
                                // <img className='avatarImage' src={`${avatars[3]}`} />
                            }
                            <div className='qr-code-btn' onClick={() => { setVisibility({ ...visibility, qrCode: true }); }}>
                                <img src={'./images/qrCodeIcon.svg'} />
                            </div>

                        </div>

                        <div className='goldCount'><span>{gold === null ? "...и ваши карманы пусты" : gold + " " + getGoldWord(gold)}</span></div>

                        <CustomButton
                            onClick={() => {
                                if (messageAboutStart === 0) {
                                    setVisibility({ ...visibility, qrScaner: true });
                                }
                                else setVisibility({ ...visibility, message: true });
                            }}
                        >
                            Взаимодействовать
                        </CustomButton>

                        <CustomButton
                            onClick={() => {
                                if (messageAboutStart === 1) {
                                    dispatch(getActions({ callback: (actions) => dispatch(setActions(actions)) }));
                                    navigator("/status");
                                }
                                else { setVisibility({ ...visibility, message: true }); }
                            }}
                        >
                            Статус
                        </CustomButton>

                    </>}
        </div>

    )
}
