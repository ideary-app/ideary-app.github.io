# Ideary — CONTEXTO

**Última actualización:** 2026-10-18 (versión 1.4.0)

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
- **Talk to your mentor** (botón en el header del proyecto): chat libre con la IA sobre todo el proyecto. Recibe todos los capítulos con sus pasos y avance, About + Key decisions, notas, cosas a resolver y lecciones. Si sugiere agregar algo (capítulo, decisión, cosa a resolver, nota) lo manda en un bloque ```actions``` y el usuario toca Add/Skip en cada una. La charla se guarda en `chat` del proyecto (últimos 30 mensajes).
- **Voz (micrófono)**: botón 🎤 en los chats de IA (mentor, tutor, Develop idea, I'm stuck), en "Your work" del paso y en Brain dump. Con key: graba audio y Gemini lo transcribe (mezcla español/inglés, con puntuación, máx. 5 min); sin key: reconocimiento de voz del navegador si existe. Mientras graba, el campo se reemplaza por barras de audio en vivo (nivel real del micrófono) y no se puede escribir; ■ para y transcribe, y Send/Enter para, transcribe y envía. Si no llega sonido o no se entienden palabras, avisa con un mensaje claro.
- **Esperas de IA visibles:** todos los chats muestran puntitos animados, segundos transcurridos y Cancel; el campo y Send quedan bloqueados mientras la IA responde o mientras se transcribe la voz. Si cerrás el chat del mentor y lo reabrís, la respuesta en curso se sigue viendo. Si falla, el error se muestra y la pregunta vuelve a la caja.
- **Header de ideas/proyectos:** no se achica al scrollear (preferencia del autor). Muestra "Last edited …" como texto (no botón). "Turn back into a single idea" y "💬 Talk to your mentor" van a la derecha de la fila de etiquetas del header del proyecto (visibles sin scrollear).
- **Bloques colapsables** (columna derecha: Things to figure out, Notes, Sketches, References): clic en el título; colapsado muestra un resumen (barra "x/y figured out", cantidad de notas/bocetos/links). Arrancan plegados; se recuerdan los que abrís, por dispositivo (`ideary_expanded_v1`).
- **Sort notes** ("Sort into the project/plan"): elegís a dónde va cada nota (Key decision, What it is/Description, Things to figure out, borrar o dejarla). Con key, "Suggest" propone destinos en streaming: cada nota muestra debajo del selector un "* por qué" breve de la IA (se tacha si cambiás el destino); se ven ordenándose una por una con contador ("Sorted 3 of 30") y barra; el usuario confirma.
- **Help me decide** (en cada cosa a resolver): abre el chat del mentor (sirve en proyectos, capítulos e ideas) pidiendo 3–5 opciones con pros/contras. Cuando elegís, el mentor ofrece "Settle …": marca la cosa como resuelta, guarda la elección (`decision` en el ítem) y la suma a Key decisions del proyecto (en ideas sueltas, como nota "Decided: …").
- **Velocidad de IA:** para tareas rápidas (ordenar notas, transcribir, chats) se pide poco "thinking" al modelo (`thinkingConfig`); si el modelo no lo acepta, se reintenta sin. Ordenar notas va en tandas de 6 con progreso real y filas bloqueadas hasta que se ordenan.
- **Check chapter order** (antes "Check chapter order"): el director revisa el orden de los capítulos y propone mover o agregar capítulos, además de avisos. El usuario aplica o ignora cada sugerencia. La app valida la coherencia del orden (ver la sección 7).
- **Explain this stage**: en cada capítulo, la IA escribe una explicación de esa etapa con tono de profesor.
- **Change log con Undo**: un historial de los cambios hechos por la IA en cada idea. Los últimos 5 cambios de IA tienen un botón "↶ Undo" que devuelve la idea (y los capítulos, si es un proyecto) a como estaba antes. Los bocetos no se tocan.
- **Bocetos**: se puede dibujar en la app o subir una foto.
- **Notas fechadas**: funcionan como bitácora del proceso.
- **Cierre**: al terminar una idea se completa qué funcionó, qué no y qué harías distinto.
- **Sincronización con Google Drive** (opcional), con combinación de cambios entre dispositivos. Los guardados se agrupan: se suben ~2,5 s después del último cambio (o al salir de la app) y un chip muestra "Saving… / ✓ Saved". Si cerrás con algo pendiente, avisa.
- **References**: links por idea/capítulo/proyecto (`referencias: [{url, titulo, fecha}]`), con miniatura si es YouTube. "What should I look for?" pide a la IA búsquedas concretas (chips que abren YouTube/Google).
- **Lecciones de proyectos terminados**: lo escrito en el cierre (`cierre`) de ideas terminadas (misma categoría primero, hasta 3) se manda a Develop idea y al director para que avisen si repetís un error.
- **Share**: Print/Save as PDF, descargar una página .html autónoma, o copiar como texto. No hay link en vivo (no hay servidor): se comparte una copia.
- **Contador de uso de IA** en Settings (pedidos de hoy y tokens, por dispositivo; `ideary_ai_usage_v1`).
- **Today** (Home): los 1–3 próximos pasos de lo que está In progress (uno por proyecto), primero lo que vence antes y después lo último trabajado. Tocar uno abre ese paso con el tutor.
- **Fechas**: fecha objetivo opcional (`fechaObjetivo`, "YYYY-MM-DD") en ideas, proyectos y capítulos (badge 📅, avisa "soon"/"overdue"). Se elige con un calendario propio con estilo cuaderno que abre pegado a la píldora (atajos: In a week / In a month / End of month); la × en la píldora borra la fecha. "Last worked on" automático (`actualizado`, ISO) y 💤 en ideas sin cambios hace 3+ semanas.
- **Botón "Next step →"** flotante en celular: abre directo el próximo paso sin terminar con el tutor.
- **Tour guiado (coach marks)**: oscurece la app y deja iluminada solo la parte que explica, con una notita al lado (Back/Next/Skip, flechas del teclado, Esc). Tour de Home (9 pasos; el último pide tocar "+ New idea" de verdad) y tours cortos la primera vez que abrís una idea con plan y un proyecto. Se recuerdan los vistos (`ideary_tours_seen_v1`); "?" y "How it works" lo repiten. Reemplaza a la bienvenida en modals.
- **Intensidad del fondo** (0,25–1, por defecto 0,75): suaviza el mármol de la tapa y los renglones/márgenes. Se ajusta deslizando el cierre de la cartuchera: la posición del cierre ES la intensidad (izquierda del todo 25%, derecha del todo 100%), o con el slider de Settings, que tiene la misma pinta del cierre y va de 1 en 1 (`ideary_bg_intensity_v1`).
- **(Anterior) Bienvenida en 3 pasos** (qué es, cómo usar la IA con o sin key —con guía para sacar la key gratis— y conectar Drive). Se abre la primera vez y desde "?".
- **Ayudas "?"** al lado de Production plan, Brain dump/I'm stuck y Check chapter order.
- **Personalización**: tema Day/Night (Day por defecto; Night es una paleta marrón oscuro con letra crema), color de tapa del cuaderno y cartuchera, que es el header en celular.
- **Exportar e importar** respaldos en JSON.
- **Aviso de nueva versión y changelog** público ("What's new").

## 2. Cómo se usa

**Pantallas**
- **Home**: es la tapa del cuaderno, de mármol con el logo. Tiene:
  - un buscador;
  - las pestañas All / Projects / Ideas, que filtran las tarjetas de estado;
  - las tarjetas de estado con la cantidad de ideas de cada uno;
  - el botón "Create new idea" justo debajo del buscador (ya no hay post-it "How it works": el tour se repite con "?");
  - un aviso para conectar Drive si no hay sesión.
- **Sidebar**: es el índice del cuaderno. Incluye:
  - Home, "+ New idea", "?" (guía), Connect Google Drive y Settings;
  - las categorías, con sus ideas debajo;
  - los proyectos con un ícono ▤. Sus capítulos se despliegan con ▸/▾ y solo se abren solos los del proyecto en el que estás;
  - Trash;
  - la versión y "What's new".

  En celular la sidebar es un menú lateral.
- **Header**: es una cartuchera con cierre. El tirador del cierre funciona como slider de intensidad del fondo.
- **Vista de idea**:
  - **Arriba**, una tarjeta fija con cinta y lomo de color. Muestra el título y los botones Edit (lápiz), Develop idea (marcador) y Erase (goma). Al scrollear se achica.
  - **En desktop hay dos columnas**:
    - a la izquierda, Progress, Production plan (fases y pasos), Brain dump e I'm stuck;
    - a la derecha, Description, Notes, Things to figure out, Sketches y Change log.
  - **En celular**, todo va apilado en una sola columna.
- **Vista de proyecto**:
  - About dividido en "What it is" (la descripción) y "Key decisions" (lista editable: Enter para agregar, clic para editar, × para borrar). Un solo "View more / View less" pliega las dos columnas juntas (plegado tiene una altura fija, la misma que el bloque con poco contenido, con fundido abajo; el botón solo aparece si hay más contenido);
  - Next up;
  - el muro de capítulos (post-its que se pueden reordenar con ← →, más "New chapter");
  - Check chapter order;
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
| Varios | `ideary_tours_seen_v1`, `ideary_bg_intensity_v1`, `ideary_expanded_v1`, `ideary_ai_usage_v1`, `ideary_home_filter_v1`, `ideary_view`, `ideary_howto_seen_v1`, `ideary_install_banner_dismissed`, `ideary_zip_x` (posición del cierre) |

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
    "referencias": [{ "url": "https://www.youtube.com/watch?v=xxxx", "titulo": "Tutorial de ensamblado", "fecha": "2026-10-12T09:00:00.000Z" }],
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
- **Respaldo automático** en Drive (descartado por ahora por el autor).
- **Link en vivo para compartir:** requeriría un servidor; hoy se comparte una copia (PDF, página .html o texto).
- **Regla de diseño de superficies:** fichas (bloques con cinta) para información; post-its (amarillo/rosa) para lo que escribe el usuario (notas, cosas a resolver, capítulos); cinta solo en elementos destacados. Respetarla en UI nueva.
- **Regla de mantenimiento:** cada cambio de la app tiene que actualizar este `CONTEXTO.md` en el mismo commit.

- **Home:** el aviso "Keep your ideas safe" (sin Drive) va a la derecha de la tarjeta del logo, que queda centrada; en pantallas angostas va debajo del logo, arriba del buscador.
- **Sidebar:** el anillado del cuaderno va a la izquierda.

- **Cartucheras (rediseño):** Pencils con lápices enteros (sin cortes); Coral con dibujitos sueltos en un patrón largo poco repetitivo; Brown leather y Low-profile black con textura de cuero en relieve; Checker, Mint stars y Rainbow con patrón más grande en desktop (Rainbow con franjas más gruesas).

- **Mobile:** los "?" de ayuda se abren con un toque y su globo se posiciona por JS para no salirse de la pantalla. "Description"/"What this stage is about" no se pliegan. Hay un script de auditoría (Playwright, 360/390px) que recorre todas las pantallas y modales buscando elementos fuera de pantalla, botones cortados y títulos rotos; correrlo antes de publicar cambios de UI.

- **Adjuntos (imágenes y archivos de texto):** en cada paso, pestaña "My work" → "📎 Add image or file" (imágenes achicadas a 1600px JPEG; texto .txt/.md/.csv/.json/.srt/.fountain hasta 120 KB) guardados en el paso (`adjuntos: [{tipo:"img"|"texto", nombre, src|texto, fecha}]`), dentro del archivo único. El tutor recibe el texto y hasta 4 imágenes (Gemini multimodal). En el chat del mentor hay 📎: la IA ve lo adjuntado y puede ofrecer "archivar" (`{"tipo":"archivar","archivo","paso"}`) para guardarlo en el paso que corresponde; si no encuentra paso, va a Images (imagen) o Notes (texto). Formatos propietarios (ej. PureRef .pur) no se leen: exportar una imagen.
- **Images** (antes "Sketches"): dibujos e imágenes propias de la idea + las imágenes adjuntas a sus pasos (marcadas 📎 con el nombre del paso; tocar abre la imagen en el visor, con botón "Keep working on this step →"). Regla para el usuario: subí la imagen donde estés trabajando (el paso); aparece también en Images.
- **Workspace del paso:** dos pestañas, "✏️ My work" (texto + adjuntos + Save / Save & mark done) y "💬 Ask the tutor" (chat; "+ Add to my work" pasa la respuesta a My work).
- **Header en desktop:** logo a la izquierda; buscador y "+" en el centro; "?" (tour) y Settings a la derecha (en desktop esos botones salen de la sidebar).
- **Tour vs. Drive:** el tour espera a que Drive termine de cargar (y no aparece con el banner de reconexión). Los tours vistos y la intensidad del fondo se guardan también en `ideary-settings.json`, así no se repiten en otros dispositivos.

- **Visor de imágenes:** header arriba con nombre y aviso de calidad ("Preview · 4096×… px — the original is 20000×11000 px"), a la derecha los controles (− % + Fit), "⬇ Download full res" (baja el original), Erase si corresponde y la ✕ a la derecha de todo. Abajo a la derecha flota "Keep working on this step →" cuando la imagen viene de un paso. Zoom con rueda / pellizco / doble toque, arrastrar para moverse, Esc cierra.

- **Almacenamiento de imágenes (desde 1.4.0):** las imágenes ya NO van dentro del JSON. Cada imagen es un objeto `{ref, driveId?, fecha, ...}` (en `bocetos`, `adjuntos` de pasos y `adj` del chat).
  - Con Drive: carpeta **`Ideary/`** en el Drive del usuario con `ideary-data.json`, `ideary-settings.json` y la subcarpeta **`images/`** (un archivo por imagen, `<ref>.jpg|png`). Se cachean en IndexedDB.
  - Sin Drive: los archivos van a IndexedDB (`ideary_images`), con lugar para cientos de MB; el JSON en localStorage queda liviano.
  - Al conectar Drive, las imágenes locales se suben solas. Las imágenes viejas guardadas como data URL (`src`) se migran automáticamente.
  - Calidad: si la imagen mide hasta 4096px (y pesa hasta 15 MB) se guarda tal cual. Si es más grande, la app usa una **preview de 4096px** (JPEG 0,9) y guarda además el **original intacto** (`origRef` en IndexedDB / `origDriveId` en `Ideary/images`, con `origW`/`origH`). A la IA se le manda una copia de 1280px.
  - Al borrar una imagen se borra también de IndexedDB y de Drive (con reintentos; si se borró mientras se subía, se borra al terminar la subida). Además hay una **limpieza automática**: cada ~10 min (tras sincronizar) se borran de `Ideary/images` los archivos que ninguna idea usa y que tienen más de 15 min (cubre ideas borradas, papelera vaciada, borrados fallidos).
  - Imágenes de más de 4,5 MB se suben a Drive con subida "resumable" (la simple corta en 5 MB).
- **Backups:** "Backup / Import" ofrece **ZIP** (`ideas.json` + carpeta `images/`, calidad completa) o **un solo JSON** con las imágenes adentro como texto (data URL). Import acepta los dos (o pegar el JSON).

- **Gemini ocupado (503):** `geminiGenerate` reintenta hasta 3 veces con espera (2s, 5s, 9s), alternando con el modelo `-lite` **solo si existe** para esa key (`geminiAvailable`, de ListModels; si igual da 404 vuelve al modelo normal); si igual falla, el mensaje muestra lo que respondió Google.

- **Mentor más involucrado:** cuando contás una idea suelta, el mentor propone dónde va: acción `nota` con `paso` (se agrega al "trabajo" de ese paso con 💡) o `capitulo` (nota del capítulo). Las key decisions quedan para cosas decididas.
- **Respuestas en streaming:** `geminiGenerate(..., onChunk)` usa `streamGenerateContent?alt=sse`; el mentor muestra el texto mientras llega y usa thinking `minimal` en modelos flash 3+.
- Si el streaming falla por red, `geminiGenerate` reintenta una vez sin streaming; el error muestra el motivo técnico.
- **Gemini ocupado:** si un modelo responde 'high demand' se marca ocupado 2 min (`geminiBusy`) y se prueba otro modelo flash/flash-lite disponible para la key (`geminiAltModel`, más nuevo primero), hasta 4 intentos.
- **Cuota (429):** la cuota gratis es por modelo; si uno da 429 se evita 1 hora y se prueba otro modelo disponible. El aviso de cuota solo sale si se agotaron todos.
