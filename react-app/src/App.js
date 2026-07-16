import React from 'react';
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import StudentList from './components/StudentList';
import StudentForm from './components/StudentForm';

function App() {
    return (
        <BrowserRouter>
            <div className="container py-5">
                <header className="mb-5 text-center">
                    <h1 className="fw-bold">SchoolDesk <span className="text-primary">ERP</span></h1>
                    <p className="text-muted">React Integration Demo</p>
                </header>

                <main>
                    <Routes>
                        <Route path="/" element={<Navigate to="/students" replace />} />
                        <Route path="/students" element={<StudentList />} />
                        <Route path="/students/new" element={<StudentForm />} />
                        <Route path="/students/edit/:id" element={<StudentForm />} />
                    </Routes>
                </main>
            </div>
        </BrowserRouter>
    );
}

export default App;
