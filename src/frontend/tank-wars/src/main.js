import {useNavigate} from 'react-router-dom';

function Main() {
    const navigate = useNavigate();
    return (
        <div>
            <h1>Plan your next move!</h1>   
            <button onClick={() => navigate('/game-editor')}>Extend Fortification</button>
            <button>Attack</button>
        </div>
    );
}

export default Main;
