-- Resonance Knowledge Seed Data

-- ── Senses ────────────────────────────────────────────────────────────────────

INSERT OR IGNORE INTO senses (id, name, emoji, description) VALUES
    ('seen',       'Seen',        '👁',  'Something witnessed — image, colour, movement, or absence of light'),
    ('heard',      'Heard',       '👂',  'Something received through sound — music, voice, noise, or silence'),
    ('felt',       'Felt',        '🤲',  'Something experienced through the body — touch, pressure, temperature, or pain'),
    ('thought',    'Thought',     '💭',  'A mental event — idea, memory, realisation, or question'),
    ('felt_inside','Felt Inside', '🫀',  'An internal state — emotion, mood, energy level, or nervous system'),
    ('dreamt',     'Dreamt',      '🌙',  'Something from dreams, half-sleep, or the hypnagogic threshold'),
    ('grateful',   'Grateful For','🙏',  'Anything noticed with appreciation, however small'),
    ('other',      'Other',       '⚪',  'Anything that does not fit the above categories');

-- ── Starter subcategories ─────────────────────────────────────────────────────

INSERT OR IGNORE INTO subcategories (id, sense_id, name, description) VALUES
    ('seen_colour',    'seen',       'Colour',     'A specific colour or quality of light'),
    ('seen_scene',     'seen',       'Scene',      'A place, landscape, or environment'),
    ('seen_face',      'seen',       'Face',       'A person or expression'),
    ('seen_symbol',    'seen',       'Symbol',     'An image, object, or pattern that carried meaning'),
    ('heard_music',    'heard',      'Music',      'A piece of music, instrument, or sound composition'),
    ('heard_voice',    'heard',      'Voice',      'A voice — speaking, singing, or silent'),
    ('heard_ambient',  'heard',      'Ambient',    'Environmental or background sound'),
    ('felt_pain',      'felt',       'Pain',       'Physical discomfort or sensation'),
    ('felt_pleasure',  'felt',       'Pleasure',   'Physical ease, comfort, or delight'),
    ('felt_energy',    'felt',       'Energy',     'Aliveness, exhaustion, or physical charge'),
    ('thought_idea',   'thought',    'Idea',       'A new thought, insight, or creative spark'),
    ('thought_memory', 'thought',    'Memory',     'Something remembered'),
    ('thought_worry',  'thought',    'Worry',      'A recurring anxious thought'),
    ('felt_emotion',   'felt_inside','Emotion',    'A named feeling state'),
    ('felt_nervous',   'felt_inside','Nervous System', 'Regulation, dysregulation, or window of tolerance'),
    ('dreamt_dream',   'dreamt',     'Dream',      'A full dream narrative or fragment'),
    ('dreamt_liminal', 'dreamt',     'Liminal',    'Hypnagogic or hypnopompic imagery'),
    ('grateful_small', 'grateful',   'Small Thing','A micro-moment of gratitude'),
    ('grateful_person','grateful',   'Person',     'Appreciation for someone');

-- ── Emoji definitions ─────────────────────────────────────────────────────────

INSERT OR IGNORE INTO emoji_definitions (id, emoji, label, category, keywords, color, sound, texture, temperature, definition) VALUES
    ('calm',       '😌', 'Calm',          'state',   '["stillness","settled","quiet","peace"]',          '#6C5CE7', 'low hum of a distant fan',    'warm fleece',         'room temperature',      'A settled stillness. The breath after a long exhale.'),
    ('energy',     '🔥', 'Energy',         'state',   '["kinetic","fire","ignition","drive"]',             '#E17055', 'crackling fire',               'dry heat on skin',    'warm',                  'Kinetic ignition. The feeling of becoming more than yourself.'),
    ('sad',        '😢', 'Sad',            'state',   '["grief","blue","tears","ache"]',                  '#74B9FF', 'rain on glass',                'damp cloth',          'cool',                  'The softness inside grief. Not collapse — presence.'),
    ('happy',      '😊', 'Happy',          'state',   '["joy","delight","light","smile"]',                '#FDCB6E', 'distant laughter',             'sunlit surface',      'gentle warmth',         'Uncomplicated delight. The kind that needs no explanation.'),
    ('overstim',   '🌀', 'Overstimulated', 'state',   '["overwhelm","too much","noise","spiral"]',        '#A29BFE', 'overlapping voices',           'buzzing surface',     'uneven',                'Too much at once. The body''s signal to reduce input.'),
    ('melancholy', '🌙', 'Melancholy',     'state',   '["wistful","longing","ache","moonlight"]',         '#636E72', 'silence between notes',        'cold stone',          'cool and still',        'Ache with beauty in it. A longing for something half-remembered.'),
    ('transcend',  '✨', 'Transcendent',   'state',   '["awe","gold","beyond","formless"]',               '#FFD700', 'ringing silence',              'weightless',          'neither warm nor cold', 'The moment experience stops being ordinary and becomes something else.'),
    ('focused',    '🎯', 'Focused',        'state',   '["concentration","locked in","clear","teal"]',     '#00CEC9', 'clean tone',                   'smooth glass',        'slightly cool',         'Everything peripheral disappears. Only the work.'),
    ('connected',  '💙', 'Connected',      'state',   '["belonging","together","bridge","blue"]',         '#0984E3', 'resonant chord',               'held hand',           'body temperature',      'The felt sense of not being alone.'),
    ('relief',     '😮‍💨', 'Relief',         'state',   '["release","exhale","unclench","mint"]',           '#55EFC4', 'long exhale',                  'releasing grip',      'cool breeze',           'The release of something you were holding without knowing.'),
    ('tired',      '💤', 'Tired',          'state',   '["exhaustion","rest","heavy","grey"]',             '#B2BEC3', 'slow breathing',               'heavy blanket',       'slightly warm',         'Bone-deep rest need.'),
    ('celebratory','🎉', 'Celebratory',    'state',   '["celebration","joy","shared","loud"]',            '#E84393', 'cheering crowd',               'confetti',            'warm and electric',     'Shared joy made loud. Belonging in sound.');
