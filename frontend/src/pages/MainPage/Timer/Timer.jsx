import React, { useState, useEffect } from 'react'
import { useDispatch, useSelector } from 'react-redux';
import { setTimer } from '../../../redux/status';
import "./Timer.scss"

export function Timer() {

    // const [timer, setTimer] = useState(null);
    const timer = useSelector((state) => state.status.timer);
    const dispatch = useDispatch();
    const [timerString, setTimerString] = useState({ minutes: "60", seconds: "00" })
    useEffect(() => {
        // let timer = 60 * 60;
        console.log(timer)

        const interval = setInterval(() => {
            if (timer !== null) {
                let tempTimer = timer
                console.log(timer)
                dispatch(setTimer(tempTimer - 1));
                const minutes = Math.floor(tempTimer / 60);
                const seconds = tempTimer % 60 < 10 ? "0" + tempTimer % 60 : tempTimer % 60;
                setTimerString({ minutes: minutes, seconds: seconds })
                // if (Object.prototype.toString.call(DateNow) === '[object Date]') {
                //     DateNow.setSeconds(DateNow.getSeconds() + 1);
                //     let TimeStr = DateNow.toTimeString();
                //     TimeStr = TimeStr.slice(0, 8);
                //     let day = DateNow.getDate();
                //     let month = DateNow.getMonth();
                //     let year = DateNow.getFullYear();
                //     // let resDate = (day < 10 ? "0" + day : day) + "-" + ((month + 1) < 10 ? "0" + (month + 1) : (month + 1)) + "/" + year + " " + TimeStr;
                //     let resDate = year + "-" + ((month + 1) < 10 ? "0" + (month + 1) : (month + 1)) + "-" + (day < 10 ? "0" + day : day) + " " + TimeStr;
                //     if (!state.connectionLoose) setDate(resDate);
                // }
            }
        }, 1000);

        return () => clearInterval(interval);
    }, [timer]);

    return (
        <div className='Timer'>
            <span>{timerString.minutes + ":" + timerString.seconds}</span>
        </div>
    )
}
