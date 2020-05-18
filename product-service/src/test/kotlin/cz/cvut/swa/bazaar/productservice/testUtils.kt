package cz.cvut.swa.bazaar.productservice

import cz.cvut.swa.bazaar.productservice.data.*
import java.math.BigDecimal
import java.time.LocalDateTime
import java.util.*

fun randomUuid() = UUID.randomUUID().toString()

fun fakeProduct(): Product = Product(
        randomUuid(),
        "testProduct",
        "Cool product",
        BigDecimal.ONE,
        1L)

fun fakeProductDTO(): ProductDTO = ProductDTO(
        "testProduct",
        "Cool product",
        BigDecimal.ONE,
        1L)

fun fakeReviewDTO(): ReviewDTO = ReviewDTO(
        1L,
        1L,
        "The product works as expected",
        randomUuid(),
        timestamp(),
        79)

fun fakeProductWithReviewsDTO(): ProductWithReviewsDTO = ProductWithReviewsDTO(
        randomUuid(),
        "testProduct",
        "Cool product",
        BigDecimal.ONE,
        1L,
        ProductStatus.AVAILABLE,
        LocalDateTime.now(),
        emptyList()
)

fun fakeProductStatusUpdateDTO(newStatus: ProductStatus): ProductStatusUpdateDTO = ProductStatusUpdateDTO(
        randomUuid(),
        newStatus
)

fun timestamp(): Int = (System.currentTimeMillis() / 1000L).toInt()