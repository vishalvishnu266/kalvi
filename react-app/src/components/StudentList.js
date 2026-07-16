import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { StudentApi } from '../api/students';

const StudentList = () => {
    const navigate = useNavigate();
    const [students, setStudents] = useState([]);
    const [loading, setLoading] = useState(true);
    const [search, setSearch] = useState('');

    const loadStudents = async () => {
        setLoading(true);
        try {
            const data = await StudentApi.list({ q: search });
            setStudents(data);
        } catch (err) {
            console.error(err);
        }
        setLoading(false);
    };

    useEffect(() => {
        loadStudents();
    }, [search]);

    const handleDelete = async (id) => {
        if (window.confirm('Are you sure you want to delete this student?')) {
            await StudentApi.delete(id);
            loadStudents();
        }
    };

    return (
        <div className="card shadow-sm">
            <div className="card-header bg-white d-flex justify-content-between align-items-center py-3">
                <h5 className="mb-0 fw-bold text-primary">Students</h5>
                <button className="btn btn-primary btn-sm" onClick={() => navigate('/students/new')}>
                    <i className="fas fa-plus me-1"></i> Add Student
                </button>
            </div>
            <div className="card-body">
                <div className="mb-3">
                    <input 
                        type="text" 
                        className="form-control" 
                        placeholder="Search students..." 
                        value={search}
                        onChange={(e) => setSearch(e.target.value)}
                    />
                </div>
                {loading ? (
                    <div className="text-center py-5">Loading...</div>
                ) : (
                    <div className="table-responsive">
                        <table className="table table-hover align-middle">
                            <thead className="table-light">
                                <tr>
                                    <th>Admission No</th>
                                    <th>Name</th>
                                    <th>Class</th>
                                    <th>Status</th>
                                    <th className="text-end">Actions</th>
                                </tr>
                            </thead>
                            <tbody>
                                {students.map(student => (
                                    <tr key={student.id}>
                                        <td><code>{student.admission_no}</code></td>
                                        <td>{student.first_name} {student.last_name}</td>
                                        <td>{student.class_name}-{student.section}</td>
                                        <td>
                                            <span className={`badge ${student.status === 'Active' ? 'bg-success-subtle text-success' : 'bg-secondary-subtle text-secondary'}`}>
                                                {student.status}
                                            </span>
                                        </td>
                                        <td className="text-end">
                                            <button className="btn btn-link btn-sm text-primary me-2" onClick={() => navigate(`/students/edit/${student.id}`)}>
                                                <i className="fas fa-edit"></i>
                                            </button>
                                            <button className="btn btn-link btn-sm text-danger" onClick={() => handleDelete(student.id)}>
                                                <i className="fas fa-trash"></i>
                                            </button>
                                        </td>
                                    </tr>
                                ))}
                                {students.length === 0 && (
                                    <tr>
                                        <td colSpan="5" className="text-center py-4 text-muted">No students found</td>
                                    </tr>
                                )}
                            </tbody>
                        </table>
                    </div>
                )}
            </div>
        </div>
    );
};

export default StudentList;
