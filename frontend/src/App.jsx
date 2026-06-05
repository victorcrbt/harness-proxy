import React, { useState, useEffect, useMemo, useRef, useCallback } from 'react';
import { Layers, Trash2, Edit2, Plus, Moon, Sun, Sliders, Save, X, ChevronLeft, ChevronRight, ArrowUpDown, Search } from 'lucide-react';

const API_URL = 'http://127.0.0.1:11436/v1/config/models';
const PAGE_SIZE_OPTIONS = [5, 10, 20, 50];

export default function App() {
  const [darkMode, setDarkMode] = useState(() => localStorage.getItem('theme') === 'dark');
  const [models, setModels] = useState([]);
  const [newModel, setNewModel] = useState("");
  const [editingIndex, setEditingIndex] = useState(null);
  const [editingValue, setEditingValue] = useState("");
  const [currentPage, setCurrentPage] = useState(1);
  const [itemsPerPage, setItemsPerPage] = useState(10);
  const [sortOrder, setSortOrder] = useState('asc');
  const [search, setSearch] = useState("");
  const editInputRef = useRef(null);

  useEffect(() => {
    if (darkMode) {
      document.documentElement.classList.add('dark');
      localStorage.setItem('theme', 'dark');
    } else {
      document.documentElement.classList.remove('dark');
      localStorage.setItem('theme', 'light');
    }
  }, [darkMode]);

  useEffect(() => {
    fetch(API_URL)
      .then(res => res.json())
      .then(data => setModels(Array.isArray(data) ? data : []))
      .catch(err => console.error("Erro ao carregar modelos:", err));
  }, []);

  useEffect(() => {
    if (editingIndex !== null && editInputRef.current) {
      editInputRef.current.focus();
    }
  }, [editingIndex]);

  const saveModels = async (updatedModels) => {
    try {
      await fetch(API_URL, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updatedModels)
      });
      setModels(updatedModels);
    } catch (err) {
      console.error("Erro ao salvar:", err);
    }
  };

  const handleAddModel = (e) => {
    e.preventDefault();
    const val = newModel.trim();
    if (val && !models.includes(val)) {
      saveModels([...models, val]);
      setNewModel("");
    }
  };

  const handleRemoveModel = (idx) => {
    const updated = models.filter((_, i) => i !== idx);
    saveModels(updated);
  };

  const handleStartEdit = (idx) => {
    setEditingIndex(idx);
    setEditingValue(models[idx]);
    setNewModel("");
  };

  const handleSaveEdit = (e) => {
    e.preventDefault();
    const val = editingValue.trim();
    if (val && editingIndex !== null && !models.includes(val)) {
      const updated = [...models];
      updated[editingIndex] = val;
      saveModels(updated);
      setEditingIndex(null);
      setEditingValue("");
    }
  };

  const handleCancelEdit = () => {
    setEditingIndex(null);
    setEditingValue("");
  };

  const toggleSort = () => {
    setSortOrder(prev => prev === 'asc' ? 'desc' : 'asc');
    setCurrentPage(1);
  };

  const handleSearch = useCallback((e) => {
    setSearch(e.target.value);
    setCurrentPage(1);
  }, []);

  const sortedModels = useMemo(() => {
    return [...models].sort((a, b) =>
      sortOrder === 'asc' ? a.localeCompare(b) : b.localeCompare(a)
    );
  }, [models, sortOrder]);

  const filteredModels = useMemo(() => {
    if (!search.trim()) return sortedModels;
    try {
      const regex = new RegExp(search, 'i');
      return sortedModels.filter(m => regex.test(m));
    } catch {
      return sortedModels.filter(m => m.toLowerCase().includes(search.toLowerCase()));
    }
  }, [sortedModels, search]);

  const totalPages = Math.max(1, Math.ceil(filteredModels.length / itemsPerPage));
  const safePage = Math.min(currentPage, totalPages);
  const startIdx = (safePage - 1) * itemsPerPage;
  const pagedModels = filteredModels.slice(startIdx, startIdx + itemsPerPage);

  return (
    <div className="flex h-screen w-screen font-sans bg-slate-50 text-zinc-900 dark:bg-zinc-950 dark:text-zinc-50 transition-colors duration-200">

      {/* SIDEBAR */}
      <aside className="w-64 shrink-0 flex flex-col justify-between border-r border-slate-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 p-4">
        <div>
          <div className="flex items-center gap-3 px-2 py-4 border-b border-dashed mb-6 border-slate-200 dark:border-zinc-700">
            <div className="bg-purple-600 text-white p-2 rounded-xl">
              <Sliders size={20} />
            </div>
            <div>
              <h2 className="font-bold text-sm tracking-wide">HARNESS</h2>
              <p className="text-xs text-slate-500 dark:text-zinc-400">Management Console</p>
            </div>
          </div>
          <nav className="space-y-1">
            <a href="#" className="flex items-center gap-3 px-3 py-2.5 rounded-xl font-medium text-sm bg-purple-600 text-white shadow-lg shadow-purple-600/20">
              <Layers size={18} /> Modelos
            </a>
          </nav>
        </div>

        <div className="flex items-center justify-between p-3 rounded-xl border border-slate-200 dark:border-zinc-800 bg-slate-100 dark:bg-zinc-950">
          <span className="text-xs font-medium text-slate-500 dark:text-zinc-400 flex items-center gap-2">
            {darkMode ? <Moon size={14} /> : <Sun size={14} />} {darkMode ? 'Dark Mode' : 'Light Mode'}
          </span>
          <button onClick={() => setDarkMode(!darkMode)} className={`w-11 h-6 flex items-center rounded-full p-1 transition-all ${darkMode ? 'bg-purple-600 justify-end' : 'bg-zinc-300 justify-start'}`}>
            <div className="bg-white w-4 h-4 rounded-full shadow-md" />
          </button>
        </div>
      </aside>

      {/* MAIN CONTENT */}
      <main className="flex-1 flex flex-col p-6 gap-4 min-w-0">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">Gerenciamento de Modelos</h1>
          <p className="text-sm text-slate-500 dark:text-zinc-400">Adicione ou remova os modelos disponíveis na interface da sua IDE.</p>
        </div>

        {/* FORMULÁRIO */}
        <section className="p-6 rounded-2xl border border-slate-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-sm">
          <h3 className="text-sm font-semibold mb-4 text-purple-600 dark:text-purple-500">
            {editingIndex !== null
              ? `Editando: ${models[editingIndex]}`
              : 'Registrar Novo Modelo'}
          </h3>

          {editingIndex !== null ? (
            <form onSubmit={handleSaveEdit} className="flex gap-3">
              <input
                ref={editInputRef}
                type="text"
                value={editingValue}
                onChange={(e) => setEditingValue(e.target.value)}
                className="flex-1 px-4 py-2.5 rounded-xl text-sm border outline-none bg-purple-50 border-purple-300 focus:border-purple-600 text-zinc-900 dark:bg-purple-950/30 dark:border-purple-700 dark:focus:border-purple-500 dark:text-zinc-100 transition-all"
              />
              <button
                type="button"
                onClick={handleCancelEdit}
                className="bg-red-500 hover:bg-red-600 text-white font-medium text-sm px-4 py-2.5 rounded-xl flex items-center gap-1.5 shadow-md"
              >
                <X size={16} /> Cancelar
              </button>
              <button
                type="submit"
                className="bg-emerald-600 hover:bg-emerald-700 text-white font-medium text-sm px-5 py-2.5 rounded-xl flex items-center gap-2 shadow-md"
              >
                <Save size={16} /> Salvar
              </button>
            </form>
          ) : (
            <form onSubmit={handleAddModel} className="flex gap-3">
              <input
                type="text"
                placeholder="Ex: openrouter/google/gemini-2.5-pro"
                value={newModel}
                onChange={(e) => setNewModel(e.target.value)}
                className="flex-1 px-4 py-2.5 rounded-xl text-sm border outline-none bg-slate-50 border-slate-200 focus:border-purple-600 text-zinc-900 dark:bg-zinc-950 dark:border-zinc-800 dark:focus:border-purple-500 dark:text-zinc-100 transition-all"
              />
              <button type="submit" className="bg-purple-600 hover:bg-purple-700 text-white font-medium text-sm px-5 py-2.5 rounded-xl flex items-center gap-2 shadow-md">
                <Plus size={16} /> Adicionar
              </button>
            </form>
          )}
        </section>

        {/* TABELA — com scroll interno e paginação fixa */}
        <section className="rounded-2xl border border-slate-200 dark:border-zinc-800 bg-white dark:bg-zinc-900 shadow-sm flex flex-col min-h-0 flex-1">
          {/* Header: título + busca + ordenação */}
          <div className="px-6 py-4 border-b border-slate-100 dark:border-zinc-800 flex items-center justify-between gap-4 shrink-0 flex-wrap">
            <span className="font-medium text-xs tracking-wider uppercase text-slate-400 dark:text-zinc-400 shrink-0">
              Modelos ({filteredModels.length})
            </span>

            <div className="flex items-center gap-2 bg-slate-50 dark:bg-zinc-950 border border-slate-200 dark:border-zinc-800 rounded-lg px-3 py-1.5 min-w-0 flex-1 max-w-xs">
              <Search size={14} className="text-slate-400 shrink-0" />
              <input
                type="text"
                placeholder="Pesquisar (regex)..."
                value={search}
                onChange={handleSearch}
                className="bg-transparent text-xs outline-none text-zinc-900 dark:text-zinc-100 w-full min-w-0"
              />
              {search && (
                <button onClick={() => { setSearch(""); setCurrentPage(1); }} className="text-slate-400 hover:text-red-500 shrink-0">
                  <X size={14} />
                </button>
              )}
            </div>

            <button
              onClick={toggleSort}
              className="flex items-center gap-1.5 text-xs font-medium text-slate-500 dark:text-zinc-400 hover:text-purple-600 dark:hover:text-purple-400 transition-colors shrink-0"
            >
              <ArrowUpDown size={14} />
              {sortOrder === 'asc' ? 'A → Z' : 'Z → A'}
            </button>
          </div>

          {/* Corpo com scroll */}
          <div className="flex-1 overflow-y-auto min-h-0">
            <div className="flex items-center justify-between px-6 py-4 bg-purple-600/5">
              <div className="flex items-center gap-3">
                <span className="font-mono text-sm font-semibold text-purple-600 dark:text-purple-400">Auto (Local)</span>
                <span className="text-[10px] uppercase font-bold tracking-wider px-2 py-0.5 rounded-md bg-purple-500/20 text-purple-600 dark:text-purple-400">Sistema</span>
              </div>
              <span className="text-xs text-slate-500 dark:text-zinc-500 italic">Protegido</span>
            </div>

            {filteredModels.length === 0 ? (
              <div className="px-6 py-12 text-center text-sm text-slate-500 dark:text-zinc-500">
                {search ? 'Nenhum modelo encontrado para esta busca.' : 'Nenhum modelo cadastrado.'}
              </div>
            ) : (
              pagedModels.map((model) => {
                const realIdx = models.indexOf(model);
                return (
                  <div key={realIdx} className="flex items-center justify-between px-6 py-4 hover:bg-slate-50 dark:hover:bg-zinc-800/50 transition-colors border-t border-dashed border-slate-100 dark:border-zinc-800">
                    <span className="font-mono text-sm truncate mr-4">{model}</span>
                    <div className="flex items-center gap-1 shrink-0">
                      <button
                        onClick={() => handleStartEdit(realIdx)}
                        disabled={editingIndex !== null}
                        className="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-zinc-800 text-slate-400 hover:text-purple-600 dark:text-zinc-500 dark:hover:text-purple-400 transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
                        title="Editar"
                      >
                        <Edit2 size={15} />
                      </button>
                      <button
                        onClick={() => handleRemoveModel(realIdx)}
                        disabled={editingIndex !== null}
                        className="p-2 rounded-lg hover:bg-slate-100 dark:hover:bg-zinc-800 text-slate-400 hover:text-red-600 dark:text-zinc-500 dark:hover:text-red-400 transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
                        title="Remover"
                      >
                        <Trash2 size={15} />
                      </button>
                    </div>
                  </div>
                );
              })
            )}
          </div>

          {/* Paginação — fixa no rodapé */}
          <div className="px-6 py-3 border-t border-slate-200 dark:border-zinc-800 flex items-center justify-between text-xs text-slate-500 dark:text-zinc-400 shrink-0 flex-wrap gap-2">
            <div className="flex items-center gap-3">
              <span>Itens por página</span>
              <select
                value={itemsPerPage}
                onChange={(e) => { setItemsPerPage(Number(e.target.value)); setCurrentPage(1); }}
                className="bg-slate-50 dark:bg-zinc-950 border border-slate-200 dark:border-zinc-800 rounded-lg px-2 py-1 text-xs outline-none focus:border-purple-600 dark:focus:border-purple-500"
              >
                {PAGE_SIZE_OPTIONS.map(n => (
                  <option key={n} value={n}>{n}</option>
                ))}
              </select>
            </div>

            <span>
              {filteredModels.length === 0
                ? 'Nenhum modelo'
                : `${startIdx + 1}–${Math.min(startIdx + itemsPerPage, filteredModels.length)} de ${filteredModels.length}`}
            </span>

            <div className="flex items-center gap-1">
              <button
                onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
                disabled={safePage <= 1}
                className="p-1.5 rounded-lg hover:bg-slate-100 dark:hover:bg-zinc-800 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
              >
                <ChevronLeft size={16} />
              </button>

              {totalPages <= 5
                ? Array.from({ length: totalPages }, (_, i) => i + 1).map(p => (
                    <button
                      key={p}
                      onClick={() => setCurrentPage(p)}
                      className={`w-7 h-7 rounded-lg text-xs font-medium transition-colors ${
                        safePage === p ? 'bg-purple-600 text-white' : 'hover:bg-slate-100 dark:hover:bg-zinc-800'
                      }`}
                    >
                      {p}
                    </button>
                  ))
                : (() => {
                    const pages = [];
                    pages.push(1);
                    if (safePage > 3) pages.push('...');
                    for (let p = Math.max(2, safePage - 1); p <= Math.min(totalPages - 1, safePage + 1); p++) {
                      pages.push(p);
                    }
                    if (safePage < totalPages - 2) pages.push('...');
                    pages.push(totalPages);
                    return pages.map((p, i) =>
                      p === '...' ? (
                        <span key={`e-${i}`} className="px-1 text-slate-400">…</span>
                      ) : (
                        <button
                          key={p}
                          onClick={() => setCurrentPage(p)}
                          className={`w-7 h-7 rounded-lg text-xs font-medium transition-colors ${
                            safePage === p ? 'bg-purple-600 text-white' : 'hover:bg-slate-100 dark:hover:bg-zinc-800'
                          }`}
                        >
                          {p}
                        </button>
                      )
                    );
                  })()}

              <button
                onClick={() => setCurrentPage(p => Math.min(totalPages, p + 1))}
                disabled={safePage >= totalPages}
                className="p-1.5 rounded-lg hover:bg-slate-100 dark:hover:bg-zinc-800 disabled:opacity-30 disabled:cursor-not-allowed transition-colors"
              >
                <ChevronRight size={16} />
              </button>
            </div>
          </div>
        </section>
      </main>
    </div>
  );
}
