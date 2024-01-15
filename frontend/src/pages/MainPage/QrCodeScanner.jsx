import React from 'react'
// import { QrReader } from 'react-qr-reader';
import { QrScanner } from 'react-scan-qr-code';
// import {QrScanner} from "react-qrcode-scanner";

export function QrCodeScanner(setData) {

    return (
        <QrScanner
        onScan={(result) => { console.log(result) }}
        onError={(error) => { console.log(error) }}

        /** Default props
        onError = (error) => console.log({error}),
        onScan = (value) => console.log({value}),
        facingMode = 'environment', // environment|face
        constraints = null, //device constraints
        onLoad = (val :{mirrorVideo, streamLabel}) => null,
        flipHorizontally = false, //flip or reflect the video output based on facing mode
        style, //section styling can be added here
        className, //classnames will be added to the section wrapper
        delay = 800, //delay between each scan
        resolution = 600, //canvas resolution
        aspectRatio = '16:9',
        showViewFinder = true,
        viewFinder = { //any valid JS-CSS can be added here
            border: '12px solid rgba(255,255,255,0.3)',
            position: 'absolute',
            borderRadius: '5px',
            width: '250px',
            height: '250px'
        }
        */
    />
        // <QrReader
        //     delay={300}
        //     onError={(error) => { console.log(error) }}
        //     onScan={(result) => { console.log(result) }}
        //     style={{ width: "100%" }}
        //     facingMode="environment"
        // />
        // <QrScanner
        //     onSuccess={(result, error) => {
        //         if (!!result) {
        //             setData(result)
        //         }

        //         if (!!error) {
        //             console.info(error.message);
        //         }
        //     }}
        //     style={{ width: '250px', height: "250px"}}
        // />
    )
}
