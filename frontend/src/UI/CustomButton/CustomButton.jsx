import "./CustomButton.scss"

const CustomButton = ({ children, someClass, ...props }) => {
    return (
        <div className='buttonContainer'>
            <button className={`CustomButton ${someClass}`} {...props}>
                {children}
            </button>
        </div>
    );
}

export default CustomButton;
