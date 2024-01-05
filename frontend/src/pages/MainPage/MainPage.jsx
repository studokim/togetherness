import React, { useState } from 'react'
import "./MainPage.scss"
import CustomButton from '../../UI/CustomButton/CustomButton'
import { useNavigate } from 'react-router-dom';

import { QRCodeSVG } from 'qrcode.react';

import { QrScanner } from '@yudiel/react-qr-scanner';


export default function MainPage() {

    const [qrCodeVisible, setQRCodeVisible] = useState(false);
    const [qrScanerVisible, setQrScanerVisible] = useState(false);


    const navigator = useNavigate();

    return (
        <div className='MainPage'>
            {
                qrCodeVisible ?
                    <div className='qr-code-substrate' onClick={(e) => { e.stopPropagation(); setQRCodeVisible(false); }}>
                        {/* <div className='qr-code'></div> */}
                        <QRCodeSVG value="https://pikabu.ru/story/belaya_vorona_1999_memnyiy_film_populyarnyiy_v_rf_v_nachale_nulevyikh__smotrim_v_1080_10672321" />
                    </div>
                    :
                    null
            }

            {
                qrScanerVisible
                    ?
                    <div className='qr-code-substrate' onClick={(e) => { e.stopPropagation(); setQrScanerVisible(false); }}>
                        <QrScanner
                            containerStyle={{ position: "absolute", height: "100%", width: "100%" }}
                            onDecode={(result) => { alert(result); navigator('/interaction'); }}
                            onError={(error) => { console.log(error?.message); alert("Ошибка. Попробуйте еще раз.."); navigator('/interaction'); }}
                        />
                    </div>
                    :
                    null
            }

            <div className='timer'>XX:XX</div>
            <div className='avatar'>
                <div className='qr-code-btn' onClick={() => { setQRCodeVisible(true) }}> 
                    <img src={'./images/qrCodeIcon.svg'} />
                </div>
            </div>
            <div className='goldCount'><span>120 G</span></div>
            <CustomButton onClick={() => {
                setQrScanerVisible(true);
                // navigator('/interaction')
            }}>
                Взаимодействовать
            </CustomButton>
        </div>
    )
}
