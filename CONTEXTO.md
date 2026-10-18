# Ideary — CONTEXTO

**Última actualización:** 2026-10-18 (versión 1.1.0)

Este archivo sirve para entender Ideary sin haber leído las charlas en las que se construyó. Ideary es una app independiente: no comparte datos con las otras apps del autor (Receater, el inventario de casa). Solo va a aparecer como una entrada más en el futuro hub común.

---

## 1. Qué es y qué hace

Ideary (*idea + diary*) es un cuaderno para ideas creativas, como un corto animado, una receta, un mueble o un juego. Sirve para anotarlas, convertirlas en un plan paso a paso con ayuda de una IA que actúa como mentor experto en ese rubro, y seguir el avance hasta terminarlas.

Es pública y gratuita, sin backend, y puede usarla cualquiera. Antes se llamaba "Artdeas".

**Funciones actuales**
- **Ideas**: título, categoría, estado, descripción, notas, fases con pasos (checkboxes) y "cosas a resolver".
- **Estados**: Idea, In progress, Paused, Discarded, Completed. Además está la Papelera, que permite restaurar.
- **Develop idea**: la IA arma o mejora el plan con fases y pasos. Cada paso dice qué hacer y cómo.
- **Brain dump**: pegás texto desordenado y la IA lo ordena en fases. Después muestra un resumen de qué cambió.
- **I'm stuck**: la IA explica un paso de otra manera.
- **Help por paso (tutor)**: cada paso tiene un espacio de trabajo con un chat que guía como un profesor, sin decidir por vos. Ayuda a redactar textos y el trabajo queda guardado en el paso.
- **Proyectos y capítulos**: una idea grande se convierte en un proyecto donde cada fase pasa a ser un capítulo, que es una idea completa. El proyecto tiene:
  - una página resumen con los capítulos como post-its;
  - un "Next up" con lo siguiente a hacer;
  - las cosas a resolver.

  La conversión se puede deshacer.
- **Key decisions**: cada proyecto tiene una lista de decisiones ya tomadas ("Software: Blender", "Duración: 1 minuto"). Todas las funciones de IA las reciben como reglas fijas. Si el proyecto no tiene ninguna, el director propone las primeras y el usuario elige cuáles guardar.
- **Filtro de la Home**: All / Projects / Ideas. Se recuerda la última opción. En los proyectos se muestra el avance de capítulos.
- **Director**: la IA revisa el orden de los capítulos y propone mover o agregar capítulos, además de avisos. El usuario aplica o ignora cada sugerencia. La app valida la coherencia del orden (ver la sección 7).
- **Explain this stage**: en cada capítulo, la IA escribe una explicación de esa etapa con tono de profesor.
- **Change log con Undo**: un historial de los cambios hechos por la IA en cada idea. Los últimos 5 cambios de IA tienen un botón "↶ Undo" que devuelve la idea (y los capítulos, si es un proyecto) a como estaba antes. Los bocetos no se tocan.
- **Bocetos**: se puede dibujar en la app o subir una foto.
- **Notas fechadas**: funcionan como bitácora del proceso.
- **Cierre**: al terminar una idea se completa qué funcionó, qué no y qué harías distinto.
- **Sincronización con Google Drive** (opcional), con combinación de cambios entre dispositivos. Los guardados se agrupan: se suben ~2,5 s después del último cambio (o al salir de la app) y un chip muestra "Saving… / ✓ Saved". Si cerrás con algo pendiente, avisa.
- **Today** (Home): los 1–3 próximos pasos de lo que está In progress (uno por proyecto), primero lo que vence antes y después lo último trabajado. Tocar uno abre ese paso con el tutor.
- **Fechas**: fecha objetivo opcional (`fechaObjetivo`, "YYYY-MM-DD") en ideas, proyectos y capítulos (badge 📅, avisa "soon"/"overdue"). "Last worked on" automático (`actualizado`, ISO) y 💤 en ideas sin cambios hace 3+ semanas.
- **Botón "Next step →"** flotante en celular: abre directo el próximo paso sin terminar con el tutor.
- **Bienvenida en 3 pasos** (qué es, cómo usar la IA con o sin key —con guía para sacar la key gratis— y conectar Drive). Se abre la primera vez y desde "?".
- **Ayudas "?"** al lado de Production plan, Brain dump/I'm stuck y Ask the director.
- **Personalización**: tema Day/Night (Day por defecto; Night es una paleta marrón oscuro con letra crema), color de tapa del cuaderno y cartuchera, que es el header en celular.
- **Exportar e importar** respaldos en JSON.
- **Aviso de nueva versión y changelog** público ("What's new").

## 2. Cómo se usa

**Pantallas**
- **Home**: es la tapa del cuaderno, de mármol con el logo. Tiene:
  - un buscador;
  - las pestañas All / Projects / Ideas, que filtran las tarjetas de estado;
  - las tarjetas de estado con la cantidad de ideas de cada uno;
  - el post-it "How it works";
  - el botón "Create new idea";
  - un aviso para conectar Drive si no hay sesión.
- **Sidebar**: es el índice del cuaderno. Incluye:
  - Home, "+ New idea", "?" (guía), Connect Google Drive y Settings;
  - las categorías, con sus ideas debajo;
  - los proyectos con un ícono ▤. Sus capítulos se despliegan con ▸/▾ y solo se abren solos los del proyecto en el que estás;
  - Trash;
  - la versión y "What's new".

  En celular la sidebar es un menú lateral.
- **Header en celular**: es una cartuchera con cierre. El tirador del cierre se puede arrastrar y la posición queda guardada.
- **Vista de idea**:
  - **Arriba**, una tarjeta fija con cinta y lomo de color. Muestra el título y los botones Edit (lápiz), Develop idea (marcador) y Erase (goma). Al scrollear se achica.
  - **En desktop hay dos columnas**:
    - a la izquierda, Progress, Production plan (fases y pasos), Brain dump e I'm stuck;
    - a la derecha, Description, Notes, Things to figure out, Sketches y Change log.
  - **En celular**, todo va apilado en una sola columna.
- **Vista de proyecto**:
  - About dividido en "What it is" (la descripción, colapsable) y "Key decisions" (lista editable: Enter para agregar, clic para editar, × para borrar);
  - Next up;
  - el muro de capítulos (post-its que se pueden reordenar con ← →, más "New chapter");
  - Ask the director;
  - a la derecha, Things to figure out, Notes, Sketches y Change log;
  - el link "Turn back into a single idea".
- **Vista de capítulo**: es igual a una idea. Además tiene:
  - el link para volver al proyecto;
  - la etiqueta "Chapter X of N";
  - la navegación al capítulo anterior y al siguiente;
  - el bloque "What this stage is about".
- **Modales**: New idea, Edit (con campos o con JSON), Develop idea (chat o prompt para copiar), el asistente para convertir en proyecto, Settings, How it works y What's new.

**Flujos principales**
1. **Crear una idea:** +, completar los datos y Develop idea. La IA arma el plan y el usuario va tildando pasos.
2. **Pasos:** el checkbox tiene tres estados: vacío, empezado (medio lleno) y hecho.
3. **Convertir una idea en proyecto:**
   1. Turn into project abre el asistente.
   2. Se ve una animación de los post-its y la estrella.
   3. El director revisa el orden y el usuario aplica u omite cada sugerencia.
   4. Se abre la página del proyecto.
4. **Sin key de Gemini:** todas las funciones de IA arman un prompt para copiar en cualquier IA. El usuario pega la respuesta en JSON y la app la repara si viene mal formada.

## 3. Diseño

**Estética:** un cuaderno escolar tipo *composition notebook*:
- papel crema con renglones y línea roja de margen;
- tapa marmolada en la home;
- sidebar como índice, con espiral;
- superficies como fichas, post-its y cinta washi;
- botones con forma de lápiz, marcador y goma;
- en celular, el header es una cartuchera con cierre.

Todo tiene un aire hecho a mano y algo rotado.

**Tipografías**
- **Patrick Hand** (`--hand`, `--serif`): títulos y textos manuscritos.
- **Henny Penny** (`--logo-font`): logo.
- **Inter / sistema** (`--sans`): texto de lectura.
- **JetBrains Mono** (`--mono`): JSON y metadatos.

**Paleta (Day)**
- tinta `#1f2430`;
- birome azul `--pen #2b4fa8`;
- papel `#fbf9f1`;
- ficha `#fffefa`;
- líneas `#e4dfcf`;
- renglón `rgba(84,130,190,.22)`;
- margen `rgba(220,90,90,.55)`;
- post-it `#fff3a8` (y rosa `#ffd9df`);
- cinta `rgba(236,214,150,.72)` (y celeste `rgba(160,196,232,.72)`);
- tapa `#121212`.

**Paleta (Night, "lámpara de escritorio")**
- tinta crema `#f1e6d2`;
- papel nogal `#241b14`;
- ficha `#2f241b`;
- líneas `#463627`;
- birome ámbar `#e6b47c` (con cartuchera de color, una versión clara de ese color);
- post-it `#5a4a24`.

**Color principal:** `--pen` sale de `--brand`, que depende de la cartuchera elegida (`html[data-case]`): mint, checker, stripes, polka, pencils, coral, black o leather. Las tapas pueden ser black, white, blue, green, red, purple o yellow.

**Reglas a respetar**
- Toda UI nueva sigue la estética del cuaderno y funciona en Day y Night.
- Tiene que funcionar a 390px de ancho sin scroll horizontal.
- La vista de idea en desktop mantiene las dos columnas.
- El texto de la UI está en inglés. La IA responde en el idioma de cada idea.
- Los contenidos se alinean a la izquierda con los bordes de los bloques. El autor es exigente con esto.
- Nada de scrolls anidados que dejen contenido inaccesible. La columna derecha queda fija solo si entra en pantalla.

## 4. Datos

**Sin cuenta:** todo queda en el `localStorage` del navegador.

**Con Google Drive:** las ideas viven solo en el Drive del usuario y en memoria. Se usa el permiso `drive.file`, así que la app solo ve los archivos que crea ella misma.

| Qué | Dónde / clave |
|---|---|
| Ideas sin cuenta | `localStorage` → `ideary_cache_v1` |
| Ideas con cuenta | Drive → `ideary-data.json` |
| Ajustes con cuenta (tapa, cartuchera, key de Gemini) | Drive → `ideary-settings.json` |
| Caché de Drive (para seguir funcionando mientras se renueva el token) | `ideary_drive_cache_v1` (guarda ideas, base de sincronización y pendientes) |
| Sesión de Drive y token | `ideary_drive_session_v1`, `ideary_gtoken_v1` |
| Key y modelo de Gemini | `ideary_gemini_key_v1`, `ideary_gemini_model_v1` |
| Tapa, cartuchera, tema | `ideary_cover_v1`, `ideary_case_v1`, `ideas_producciones_tema` |
| Varios | `ideary_home_filter_v1`, `ideary_view`, `ideary_howto_seen_v1`, `ideary_install_banner_dismissed`, `ideary_zip_x` (posición del cierre) |

Las claves viejas `artdeas_*` y los archivos `artdeas-*` se migran solos la primera vez que se carga la app.

**Sincronización entre dispositivos:** cada dispositivo recuerda cómo estaba Drive la última vez que sincronizó. Al guardar, vuelve a leer Drive y combina los cambios: ganan las ideas que cambió ese dispositivo y el resto se toma de Drive. Al volver a la app, trae los cambios de los otros dispositivos.

**Estructura:** un array de ideas. Los nombres de campo están en español y **no se pueden renombrar**, porque los datos existentes dependen de ellos.

```json
[
  {
    "id": "i_1712345678",
    "titulo": "Silla de madera",
    "categoria": "Carpintería",
    "estado": "en-curso",
    "descripcion": "Una silla simple de pino para el balcón.",
    "notas": [{ "texto": "Comprar lija 120", "fecha": "2026-10-10T14:00:00.000Z" }],
    "fases": [
      {
        "nombre": "Diseño",
        "estado": "done",
        "pasos": [
          { "texto": "Medir la altura del asiento", "explicacion": "Unos 45 cm desde el piso.", "hecho": 2 },
          { "texto": "Dibujar el plano", "hecho": 1, "trabajo": "Boceto en hoja A4", "chat": [] }
        ]
      }
    ],
    "cosasAResolver": [{ "texto": "¿Qué barniz uso?", "resuelto": false }],
    "bocetos": [{ "src": "data:image/png;base64,...", "fecha": "2026-10-11T10:00:00.000Z" }],
    "fechaObjetivo": "2026-11-30",
    "actualizado": "2026-10-12T09:00:00.000Z",
    "historial": [{ "fecha": "2026-10-12T09:00:00.000Z", "fuente": "Brain dump", "cambios": ["Added phase \"Lijado\""] }]
  },
  {
    "id": "p_1",
    "tipo": "proyecto",
    "titulo": "Corto animado",
    "categoria": "Animación",
    "estado": "en-curso",
    "descripcion": "…",
    "decisiones": [{ "texto": "Software: Blender" }],
    "sugerencias": [],
    "_fasesOriginales": []
  },
  {
    "id": "c_1",
    "proyectoId": "p_1",
    "orden": 1,
    "titulo": "Guion",
    "categoria": "Animación",
    "estado": "idea",
    "fases": []
  }
]
```

**Valores de los campos**
- **`estado`:** `idea` | `en-curso` | `pausada` | `descartada` | `completada` | `eliminada`. Este último significa que está en la papelera, con `_deletedAt` y `_prevEstado`.
- **`hecho`:** 1 = empezado, 2 = hecho.
- **Otros campos opcionales:** `cierre: {funciono, noFunciono, distinto, fecha}`.
- **Proyectos y capítulos:**
  - un proyecto tiene `tipo: "proyecto"`;
  - un capítulo es una idea con `proyectoId` y `orden`;
  - al borrar un proyecto, sus capítulos se van a la papelera marcados con `_trashedWithProject`.

## 5. IA

- **Qué usa:** Google Gemini, con la key de cada usuario. El modelo se elige al usarla: el "flash" más nuevo que tenga habilitado esa key.
- **Para qué la usa:** Develop idea (chat), Brain dump, I'm stuck, el tutor de cada paso, el director de proyectos y "Explain this stage". En todas, si la idea pertenece a un proyecto, se envían las Key decisions como reglas que no se pueden contradecir (`projAbout()`).
- **Dónde está la key:** en el navegador (`ideary_gemini_key_v1`) y, si hay sesión, en `ideary-settings.json` dentro del Drive del usuario. Nunca se registra en logs ni se manda a otro lugar que no sea la API de Google.
- **Sin key:** cada función arma un prompt para copiar en cualquier IA y acepta la respuesta en JSON. `parseJsonLenient()` la repara, así que el usuario nunca tiene que arreglar JSON a mano.
- **Robustez:**
  - timeout de 90 s;
  - si el servicio está saturado (503), reintenta con la variante `-flash-lite`;
  - muestra mensajes claros si se agotó la cuota (429) o si la key es inválida.
- **Rol de la IA:** actúa como un veterano del rubro, por ejemplo un director en un corto o un chef en una receta. Guía como un profesor y no decide por el usuario.

## 6. Publicación

- **Repo:** `ideary-app/ideary-app.github.io`, en la organización de GitHub `ideary-app`, rama `main`.
- **Hosting:** GitHub Pages en **https://ideary-app.github.io/**.
- **Archivos:** `index.html`, que es toda la app con HTML, CSS y JS inline, más `privacy.html`, `terms.html` y este `CONTEXTO.md`.
- **Cómo se publica:** editar `index.html`, actualizar `DEPLOY_TIMESTAMP` y hacer push a `main`. GitHub Pages lo publica solo. La app compara `DEPLOY_TIMESTAMP` con el archivo publicado para mostrar el aviso "New version available".
- **Changelog:** los cambios visibles para el usuario se agregan a `CHANGELOG_DATA` y se sube la versión que se ve en la sidebar.
- **Google OAuth:** el client ID de Google está en el código (`GOOGLE_CLIENT_ID`), con permiso `drive.file` y email.

## 7. Decisiones tomadas (y por qué)

- **Day es el tema por defecto** (no se sigue el modo oscuro del sistema). El autor prefiere Day y pidió el Night en tonos marrones cálidos, no violeta/azul.
- **Guardado agrupado en vez de inmediato:** menos subidas del archivo entero; mientras tanto se guarda una copia en la caché local de Drive.
- **Todo en un solo archivo de datos** (`ideary-data.json`, bocetos incluidos): el autor prefiere no partirlo en varios archivos para evitar complicaciones.

- **Un solo archivo HTML y sin backend:** es simple de publicar en Pages y no junta datos de nadie.
- **Campos en español que no se renombran:** los datos guardados por los usuarios dependen de esos nombres.
- **Con Drive conectado, las ideas no se guardan en `localStorage`:** así no quedan visibles para la próxima persona que use el mismo navegador.
- **Sincronización con combinación de cambios, no "el último guarda todo":** antes, un celular con una copia vieja pisaba cambios hechos en la compu y un proyecto volvía a ser una idea.
- **El director valida el orden en la app, no solo en la IA:** el orden incoherente arruinaría el propósito de la app (por ejemplo, tapizar una silla antes de hacerle las patas). Funciona así:
  - la IA tiene que declarar de qué depende cada capítulo;
  - la app reordena lo que esté fuera de orden, corrige agregados mal ubicados y descarta las sugerencias incoherentes;
  - los avisos ("Heads-up") nunca cambian nada solos.
- **Key decisions como reglas fijas para la IA:** evitan que la IA se vaya por las ramas o contradiga lo ya definido. La IA puede proponerlas, pero solo se guardan si el usuario las acepta.
- **La IA sugiere y el usuario decide:** cada cambio del director se aplica o se ignora uno por uno. Los cambios de la IA quedan en el Change log.
- **Los capítulos no aparecen sueltos en la Home:** solo se ven dentro de su proyecto o en la sidebar debajo del proyecto.
- **La lista de ideas por defecto (`IDEAS_DEFAULT`) queda vacía:** la app es pública y no debe traer datos personales de nadie.
- **Los mensajes de error de la IA son claros, y la conversión a proyecto no depende de la IA:** el proyecto se crea igual aunque Gemini esté saturado.
- **Commits de Claude:** llevan el trailer `Co-Authored-By` y `Claude-Session`.

## 8. Pendientes e ideas a futuro

- **Integración con el hub:** solo como acceso desde el hub común. Sin compartir datos.
- **Por revisar:** los márgenes del header en algunas pantallas de desktop. Se ajustaron, pero hay que confirmarlo con el autor.
- **Regla de mantenimiento:** cada cambio de la app tiene que actualizar este `CONTEXTO.md` en el mismo commit.
