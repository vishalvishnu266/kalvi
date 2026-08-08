import { useNavigate, useParams, useSearchParams, useLocation } from 'react-router-dom';
import PageHeader from '../components/PageHeader';

export default function TaskDetailPage() {
  const params = useParams();
  const navigate = useNavigate();
  const location = useLocation();
  const [search] = useSearchParams();
  const id = String(params.id ?? '');
  const ref = search.get('ref') ?? '';

  return (
    <div className="page">
      <PageHeader title={`Task #${id}`} subtitle="Opened via deep link or navigation" />
      <section className="section">
        <div className="kv"><span>Task ID</span><code>{id}</code></div>
        {ref && (<div className="kv"><span>Ref (query param)</span><code>{ref}</code></div>)}
        <div className="kv"><span>Route path</span><code>{location.pathname}{location.search}</code></div>
        <p className="note">
          Try opening <code>dailygig://task/{id}?ref=push</code> from adb or a
          note-taking app on the phone.
        </p>
        <button className="btn" style={{ marginTop: 16 }} onClick={() => navigate(-1)}>← Back</button>
      </section>
    </div>
  );
}
