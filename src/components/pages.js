import { useState } from 'react';
import { Connection, PublicKey, Transaction, TransactionInstruction } from '@solana/web3.js';


const connection = new Connection('https://api.devnet.solana.com');
const programId = new PublicKey('');


const MainPage = () => {
    return (
        <div className="main-page">
            {/* Sidebar */}
            <div className="sidebar">
                {/* User account section */}
                <div className="user-account">
                    {/* Display user account information */}
                    <h3>User Account</h3>
                    {/* Placeholder user account information */}
                    <p>User Image</p>
                    <p>User Name</p>
                    <p>User Address</p>
                    <p>Followers: 0</p>
                </div>
                {/* Menu */}
                <div className="menu">
                    {/* Define menu items */}
                    <ul>
                        <li>Connect Wallet</li>
                        <li>Registration Page</li>
                        <li>Main Page</li>
                        <li>User Profile Page</li>
                    </ul>
                </div>
            </div>
            {/* Content */}
            <div className="content">
                {/* Tabs */}
                <div className="tabs">
                    <button>Recent Posts</button>
                    <button>NFTs</button>
                    <button>Transactions</button>
                </div>
                {/* Main content area */}
                <div className="main-content">
                    {/* Display content based on active tab */}
                    <h1>Recent Posts</h1>
                </div>
            </div>
        </div>
    );
}


const RegistrationPage = () => {
    const [name, setName] = useState('');
    const [surname, setSurname] = useState('');
    const [age, setAge] = useState('');
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');

    const handleSubmit = async (e) => {
        e.preventDefault();
        const registrationData = {
            name,
            surname,
            age,
            email,
            password
        };
        try {
            const fromAccount = new Account();
            const instructionData = Buffer.from(JSON.stringify(registrationData));
            const transaction = new Transaction().add(
                new TransactionInstruction({
                    keys: [{ pubkey: fromAccount.publicKey, isSigner: true, isWritable: false }],
                    programId,
                    data: instructionData,
                })
            );
            const signature = await connection.sendTransaction(transaction, [fromAccount]);
            console.log('Transaction sent:', signature);
        } catch (error) {
            console.error('Error sending registration data:', error);
        }
    }

    return (
        <div className="registration-page">
            <h1>Registration Page</h1>
            <form onSubmit={handleSubmit}>
                {/* Name input */}
                <label>Name:</label>
                <input type="text" value={name} onChange={(e) => setName(e.target.value)} required />
                {/* Surname input */}
                <label>Surname:</label>
                <input type="text" value={surname} onChange={(e) => setSurname(e.target.value)} required />
                {/* Age input */}
                <label>Age:</label>
                <input type="number" value={age} onChange={(e) => setAge(e.target.value)} required />
                {/* Email input */}
                <label>Email:</label>
                <input type="email" value={email} onChange={(e) => setEmail(e.target.value)} required />
                {/* Password input */}
                <label>Password:</label>
                <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} required />
                {/* Submit button */}
                <button type="submit">Register</button>
            </form>
        </div>
    );
}

export { MainPage, RegistrationPage };
