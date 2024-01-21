import React from 'react';
import "./Message.scss";

function getStringMessage(message) {
    if (message === -1) return "Ночь интриг еще не началась."
    if (message === 0) return "Ночь интриг еще не закончилась."
    if (message === 1) return "Ночь интриг закончилась."
}

const Message = ({ message, close }) => {
    return (
        <div className='darkArea' onClick={() => { close() }}>
            <div className='MessageContainer'>
                <span>
                    {getStringMessage(message)}
                </span>
            </div>
        </div>
    );
}

export default Message;
