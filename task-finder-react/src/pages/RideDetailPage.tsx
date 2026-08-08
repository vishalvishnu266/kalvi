import { useNavigate, useParams, useLocation } from 'react-router-dom';
import PageHeader from '../components/PageHeader';

export default function RideDetailPage() {
  const params = useParams();
  const navigate = useNavigate();
  const location = useLocation();
  const id = String(params.id ?? '');

  return (
    <div className="page">
      <PageHeader title={`Ride #${id}`} subtitle="Opened via deep link or navigation" />
      <section className="section">
        <div className="kv"><span>Ride ID</span><code>{id}</code></div>
        <div className="kv"><span>Route path</span><code>{location.pathname}{location.search}</code></div>
        <button className="btn" style={{ marginTop: 16 }} onClick={() => navigate(-1)}>← Back</button>
      </section>
    </div>
  );
}
