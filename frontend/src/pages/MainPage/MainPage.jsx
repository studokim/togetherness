import React, { useEffect, useState } from 'react'
import "./MainPage.scss"
import CustomButton from '../../UI/CustomButton/CustomButton'
import { useNavigate } from 'react-router-dom';

import { QRCodeSVG } from 'qrcode.react';

import { QrScanner } from '@yudiel/react-qr-scanner';
import { useSelector } from 'react-redux';
import InteractionPage from '../InteractionPage/InteractionPage';
import { Timer } from './Timer/Timer';


export default function MainPage() {

    const [visibility, setVisibility] = useState({
        qrCode: false,
        qrScaner: false,
        // ineraction: false,
    })
    const id = useSelector((state) => state.status.id);
    const name = useSelector((state) => state.status.name);
    const avatars = useSelector((state) => state.status.avatars);
    const selectedAvatar = useSelector((state) => state.status.selectedAvatar);
    const [targetId, setTargetId] = useState(null);

    const navigator = useNavigate();

    useEffect(() => {
        console.log("ID", id)
    }, [id])

    return (
        <div className='MainPage'>
            {//_______ ОТОБРАЖАЕМ ЛИБО СТРАНИЦУ ВЗАИМОДЕЙСТВИЯ ЛИБО ГЛАВНУЮ СТРАНИЦУ _________//
                targetId !== null   //если получили шв другого игроока
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
                                    <QRCodeSVG value={id} />
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
                                        onError={(error) => { console.log(error?.message); setTargetId("1704886751234"); }}
                                    />
                                </div>
                                :
                                null
                        }

                        <Timer />
                        <h1>{name}</h1>
                        <div className='avatar'>
                            <img className='avatarImage' src={`${avatars[selectedAvatar]}`} />

                            <div className='qr-code-btn' onClick={() => { setVisibility({ ...visibility, qrCode: true }); }}>
                                <img src={'./images/qrCodeIcon.svg'} />
                            </div>
                        </div>
                        <div className='goldCount'><span>120 G</span></div>
                        <CustomButton onClick={() => { setVisibility({ ...visibility, qrScaner: true }); }}>
                            Взаимодействовать
                        </CustomButton>
                        <CustomButton onClick={() => { console.log("subgect", id) }}>STATUS</CustomButton>
                    </>}
        </div>

    )
}
