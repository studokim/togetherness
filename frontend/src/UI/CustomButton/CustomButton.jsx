import "./CustomButton.scss"

const CustomButton = ({ children, someClass, size, ...props }) => {
    return (
        <div className={`buttonContainer ${size}`}>
            <button className={`CustomButton ${someClass} ${size}`} {...props}>
                {children}
            </button>
        </div>
    );
}

export default CustomButton;
