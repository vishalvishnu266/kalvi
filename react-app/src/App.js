import React, { useState } from 'react';
import StudentList from './components/StudentList';
import StudentForm from './components/StudentForm';

function App() {
    const [view, setView] = useState('list'); // 'list' or 'form'
    const [editingStudent, setEditingStudent] = useState(null);

    const handleEdit = (student) => {
        setEditingStudent(student);
        setView('form');
    };

    const handleAdd = () => {
        setEditingStudent(null);
        setView('form');
    };

    const handleSave = () => {
        setView('list');
    };

    return (
        <div className="container py-5">
            <header className="mb-5 text-center">
                <h1 className="fw-bold">SchoolDesk <span className="text-primary">ERP</span></h1>
                <p className="text-muted">React Integration Demo</p>
            </header>

            <main>
                {view === 'list' ? (
                    <StudentList onEdit={handleEdit} onAdd={handleAdd} />
                ) : (
                    <StudentForm 
                        student={editingStudent} 
                        onSave={handleSave} 
                        onCancel={() => setView('list')} 
                    />
                )}
            </main>
        </div>
    );
}

export default App;
