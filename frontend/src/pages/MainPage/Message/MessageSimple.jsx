import React from 'react';
import "./Message.scss";


const MessageSimple = ({ message, close }) => {
    return (
        <div className='darkArea' onClick={() => { close() }}>
            <div className='MessageContainer'>
                <span>
                    {message}
                </span>
            </div>
        </div>
    );
}

export default MessageSimple;
