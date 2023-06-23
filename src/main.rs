fn main() {
    println!("Hello, world!");
    let a = -1.0;
    let b = 1.0;
    let abserr = 1e-2;
    let relerr = 1e-3;
    let quancresult = quanc8(fun, a, b, abserr, relerr);

    println!("{:?}", quancresult);
}

fn fun(x: f64) -> f64 {
    x
}

/*
Оценить интервал для  FUN(X) от А до В с заданной пользователемточностью
Автоматическая адаптивная программа основанная на формуле Ньютона-Котеса 8-го порядка
*/

/*
ВХОДНАЯ ИНФОРМАЦИЯ:
FUN     - имя подпрограммы-функции FUN(X), реализующей подинтегральную функцию
А       - нижний предел интегрирования
B       - верхний предел интегрирования (может быть что  B < A)
ABSERR  - граница абсолютной погрешности (должна быть неотрицательной)
RELERR  - граница относительной погрешности (должна быть неотрицательной)
*/
/*
ВЫХОДНАЯ ИНФОРМАЦИЯ:
RESULT  - приближение к интегралу, удовлетворяющее, менее жесткой из двух границ погрешности
ERREST  - оценка величины действительной ошибки
NOFUN   - число значений функции, использованных при вычислении RESULT
FLAG    - индикатор надежности. Если  FLAG = 0, то RESULT, вероятно,
    удовлетворяет    заданной    границе   погрешности.
    Если FLAG = XXX.YYY, то ХХХ = ЧИСЛО ИНТЕРВАЛОВ, для которых
    не было сходимости, а 0.YYY = ЧАСТЬ ОСТАЛЬНОГО ИНТЕРВАЛА,
    оставшаяся  для  обработки  в  тот  момент, когда программа
    приблизилась к предельному значению для NOFUN.
*/
fn quanc8(
    fun: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    abserr: f64,
    relerr: f64,
) -> (f64, f64, i32, f64) {
    let mut result = 0.;
    let mut errest = 0.;
    let mut nofun = 0;
    let mut flag = 0.;

    let w0: f64;
    let w1: f64;
    let w2: f64;
    let w3: f64;
    let w4: f64;

    let mut area = 0.;
    let mut x0: f64;
    let mut f0: f64;
    let stone: f64;
    let mut step: f64;
    let mut cor11 = 0.;
    let mut temp: f64;

    let mut qprev = 0.;
    let mut qnow: f64;
    let mut qdiff: f64;
    let mut qleft: f64;
    let mut esterr: f64;
    let mut tolerr: f64;

    let mut qright = [0.; 31];
    let mut f = [0.; 16];
    let mut x = [0.; 16];

    let mut fsave = [[0.; 30]; 8];
    let mut xsave = [[0.; 30]; 8];

    /*
    ЭТАП 1
    Присвоение начальных значений переменным, независящим от интервала,
    генерирование констант
    */

    let levmin = 1;
    let mut levmax = 30;
    let levout = 6;
    let nomax = 5000;
    let mut nofin = nomax - 8 * (levmax - levout + 2_i32.pow((levout + 1).try_into().unwrap()));
    let mut lev = 0;
    let mut nim = 0;
    //let mut i = 0;
    //let mut j = 0;

    // ECЛИ NOFUN ДOCTИГAET ЗHAЧEHИЯ NOFIN, TO TPEBOГA

    w0 = 3956. / 14175.;
    w1 = 23552. / 14175.;
    w2 = -3712. / 14175.;
    w3 = 41984. / 14175.;
    w4 = -18160. / 14175.;

    // ПPИCBOИTЬ HУЛEBЫE ЗHAЧEHИЯ ПEPEMEHHЫM CУMMAM
    // flag = result = cor11 = * errest = area = 0.;
    // nofun = 0;
    if a == b {
        return (result, errest, nofun, flag);
    }

    /*
    ЭТАП 2
    Присвоение начальных значений переменным, зависящим от интервала,
    в соответствии с первым интервалом.
    */
    // lev = 0;
    // nim = 1;
    // qprev = 0.;
    x0 = a;
    x[15] = b;
    f0 = fun(x0);
    stone = (b - a) / 16.;
    x[7] = (x0 + x[15]) / 2.;
    x[3] = (x0 + x[7]) / 2.;
    x[11] = (x[7] + x[15]) / 2.;
    x[1] = (x0 + x[3]) / 2.;
    x[5] = (x[3] + x[7]) / 2.;
    x[9] = (x[7] + x[11]) / 2.;
    x[13] = (x[11] + x[15]) / 2.;
    for j in (1..16).step_by(2) {
        f[j] = fun(x[j]);
    }
    nofun = 9;

    'main_calc: loop {
        /*
        ЭТАП 3
        Основные вычисления
        Требуются  : QPREV,X0,X2,X4,...,X16,F0,F2,F4,...,F16.
        Вычисляются: X1,X3,...,X15,F1,F3,...,F15, QLEFT,QRIGHT,QNOV,QDIFF,AREA.
        */
        x[0] = (x0 + x[1]) / 2.;
        f[0] = fun(x[0]);
        for j in (2..=15).step_by(2) {
            x[j] = (x[j - 1] + x[j + 1]) / 2.;
            f[j] = fun(x[j]);
        }
        nofun += 8;
        step = (x[15] - x0) / 16.0;
        qleft = (w0 * (f0 + f[7])
            + w1 * (f[0] + f[6])
            + w2 * (f[1] + f[5])
            + w3 * (f[2] + f[4])
            + w4 * f[3])
            * step;
        qright[lev] = (w0 * (f[7] + f[15])
            + w1 * (f[8] + f[14])
            + w2 * (f[9] + f[13])
            + w3 * (f[10] + f[12])
            + w4 * f[11])
            * step;

        qnow = qleft + qright[lev];
        qdiff = qnow - qprev;
        area += qdiff;

        /*
        ЭТАП 4
        Проверка сходимости для интервала
        */
        esterr = qdiff.abs() / 1023.;
        tolerr = f64::max(abserr, relerr * area.abs() - 1.) * (step / stone);

        if lev < levmin {
            // ЭТАП 5 **** Сходимости нет.  Установить следующий интервал
            nim = 2 * nim;
            lev = lev + 1;
            // Запомнить элементы, относящиеся к правой половине
            // интервала, для будущего  использования.
            for i in 0..8 {
                fsave[i][lev - 1] = f[i + 8];
                xsave[i][lev - 1] = x[i + 8];
            }
            // Cобрать элементы, относящиеся к левой половине
            // интервала, для немедленного использования.
            qprev = qleft;
            for i in 1..=8 {
                let j = -i as i32;
                f[(2 * j + 17) as usize] = f[(j + 8) as usize];
                x[(2 * j + 17) as usize] = x[(j + 8) as usize];
            }
            continue 'main_calc;
        }
        if lev >= levmax as usize {
            flag += 1.;
            // ЭТАП 7
            // Сходимость для интервала имеет место.
            // Прибавить очередные слагаемые к переменным суммам.
            result += qnow;
            errest += esterr;
            cor11 += qdiff / 1023.;
            // Установить следующий интервал
            while nim != 2 * (nim / 2) {
                nim = nim / 2;
                lev = lev - 1;
            }
            nim += 1;
            if lev > 0 {
                // Собрать элементы, необходимые для следующего интервала
                qprev = qright[lev - 1];
                x0 = x[15];
                f0 = f[15];
                for i in 1..=8 {
                    f[2 * i - 1] = fsave[i - 1][lev - 1];
                    x[2 * i - 1] = xsave[i - 1][lev - 1];
                }
                continue 'main_calc;
            } else {
                break 'main_calc;
            }
        }
        if nofun > nofin {
            // ЭТАП 6
            // "Пожарный" раздел
            // Число значений функции близко к тому, чтобы превысить установленный предел.
            nofin *= 2;
            levmax = levout;
            flag += (b - x0) / (b - a);

            // ЭТАП 7
            // Сходимость для интервала имеет место.
            // Прибавить очередные слагаемые к переменным суммам.
            result += qnow;
            errest += esterr;
            cor11 += qdiff / 1023.;
            // Установить следующий интервал
            while nim != 2 * (nim / 2) {
                nim = nim / 2;
                lev = lev - 1;
            }
            nim += 1;
            if lev > 0 {
                // Собрать элементы, необходимые для следующего интервала
                qprev = qright[lev - 1];
                x0 = x[15];
                f0 = f[15];
                for i in 1..=8 {
                    f[2 * i - 1] = fsave[i - 1][lev - 1];
                    x[2 * i - 1] = xsave[i - 1][lev - 1];
                }
                continue 'main_calc;
            } else {
                break 'main_calc;
            }
        }
        if esterr <= tolerr {
            // ЭТАП 7
            // Сходимость для интервала имеет место.
            // Прибавить очередные слагаемые к переменным суммам.
            result += qnow;
            errest += esterr;
            cor11 += qdiff / 1023.;
            // Установить следующий интервал
            while nim != 2 * (nim / 2) {
                nim = nim / 2;
                lev = lev - 1;
            }
            nim += 1;
            if lev > 0 {
                // Собрать элементы, необходимые для следующего интервала
                qprev = qright[lev - 1];
                x0 = x[15];
                f0 = f[15];
                for i in 1..=8 {
                    f[2 * i - 1] = fsave[i - 1][lev - 1];
                    x[2 * i - 1] = xsave[i - 1][lev - 1];
                }
                continue 'main_calc;
            } else {
                break 'main_calc;
            }
        }
    }
    // ЭТАП 8
    // Заключительные операции и выход
    result += cor11;
    // Обеспечить, чтобы значение переменной  ERREST было не меньше уровня округлений
    if errest != 0.0 {
        temp = result.abs() + errest;
        while temp == result.abs() {
            errest *= 2.;
            temp = result.abs() + errest;
        }
    }

    (result, errest, nofun, flag)
}
